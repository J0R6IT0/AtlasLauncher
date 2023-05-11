use futures::StreamExt;
use md5;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::{
    env,
    fs::{self, File},
    io::{self, Cursor, Read, Write},
    path::Path,
    path::PathBuf,
};
use tokio::time::{sleep, Duration};
use zip::{read::ZipFile, write::FileOptions, ZipArchive, ZipWriter};

use super::{
    directory::{self, check_directory_sync},
    log::write_line,
};

pub enum ChecksumType {
    SHA1,
    SHA256,
    MD5,
}

// Read

pub async fn read_as_vec(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let exe_path: PathBuf = env::current_exe().unwrap();
    let file_path: PathBuf = exe_path.parent().unwrap().join(path);

    if !file_path.exists() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "The file does not exist",
        )));
    }

    let contents: Vec<u8> = fs::read(&file_path)?;

    Ok(contents)
}

pub async fn read_as_value<T>(path: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let bytes: Vec<u8> = read_as_vec(path).await.unwrap();
    let result: T = serde_json::from_slice(&bytes)?;

    Ok(result)
}

// Write

pub fn write_vec(data: &Vec<u8>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let exe_path: PathBuf = env::current_exe()?;

    let file_path: PathBuf = exe_path.parent().unwrap().join(path);
    let path: &Path = file_path.parent().unwrap();
    directory::check_directory_sync(path.to_str().unwrap());

    let mut file: File = File::create(&file_path)?;

    file.write_all(data)?;

    Ok(())
}

pub fn write_value<T: Serialize>(data: &T, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bytes: Vec<u8> = serde_json::to_vec(&data).unwrap();
    write_vec(&bytes, path)?;

    Ok(())
}

// Download

pub async fn download_as_vec(
    url: &str,
    checksum: &str,
    checksum_type: &ChecksumType,
    path: &str,
    extract: bool,
    force: bool,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if !force {
        let mut vec: Option<Vec<u8>> = None;

        match read_as_vec(path).await {
            Ok(file_vec) => vec = Some(file_vec),
            Err(_) => {}
        }

        if vec.is_some() {
            let vec: Vec<u8> = Some(vec).unwrap().unwrap();
            if verify_hash(checksum, checksum_type, &vec).await? {
                return Ok(vec);
            }
        }
    }

    let mut retry_count: u8 = 0;
    let bytes: Vec<u8>;

    loop {
        let response: Response = match reqwest::get(url).await {
            Ok(response) => response,
            Err(error) => {
                if retry_count >= 5 {
                    return Err(Box::new(error));
                }
                retry_count += 1;
                sleep(Duration::from_secs((1 + retry_count).into())).await;
                println!("retrying {} {url}", retry_count);
                continue;
            }
        };

        let mut current_bytes: Vec<u8> = vec![];
        let mut stream = response.bytes_stream();
        let mut stream_retry_count: u8 = 0;
        while let Some(chunk) = stream.next().await {
            let chunk = match chunk {
                Ok(chunk) => chunk,
                Err(err) => {
                    write_line(
                        &(err.to_string() + " retrying: " + &stream_retry_count.to_string() + " " + url),
                    );
                    if stream_retry_count > 10 {
                        return Err(Box::new(err));
                    }
                    stream_retry_count += 1;
                    sleep(Duration::from_secs((1 + stream_retry_count).into())).await;
                    continue;
                }
            };
            current_bytes.extend_from_slice(&chunk);
        }

        if verify_hash(checksum, checksum_type, &current_bytes).await? {
            bytes = current_bytes;
            break;
        }
        retry_count += 1;
        sleep(Duration::from_secs((1 + retry_count).into())).await;
    }

    if path != "" {
        if extract {
            let mut reader: Cursor<Vec<u8>> = Cursor::new(bytes.to_vec());
            let path: PathBuf = check_directory_sync(path);
            extract_zip(path, &mut reader).await?;
        } else {
            write_vec(&bytes.to_vec(), path)?;
        }
    }

    Ok(bytes.to_vec())
}

pub async fn download_as_json(
    url: &str,
    checksum: &str,
    checksum_type: &ChecksumType,
    path: &str,
    extract: bool,
    force: bool,
) -> Result<Value, Box<dyn std::error::Error>> {
    let vec: Vec<u8> = download_as_vec(url, checksum, checksum_type, path, extract, force).await?;
    let json: Value = serde_json::from_slice(&vec)?;
    Ok(json)
}

pub async fn download_as_string(
    url: &str,
    checksum: &str,
    checksum_type: &ChecksumType,
    path: &str,
    extract: bool,
    force: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let vec: Vec<u8> = download_as_vec(url, checksum, checksum_type, path, extract, force).await?;
    let string: String = String::from_utf8(vec)?;
    Ok(string)
}

// Verify

pub async fn verify_hash(
    checksum: &str,
    checksum_type: &ChecksumType,
    data: &[u8],
) -> Result<bool, Box<dyn std::error::Error>> {
    if checksum.is_empty() {
        return Ok(true);
    };

    let actual_checksum: String;

    let mut bytes: &[u8] = data;

    match checksum_type {
        ChecksumType::SHA1 => {
            let mut hasher = Sha1::new();
            io::copy(&mut bytes, &mut hasher)?;
            actual_checksum = format!("{:x}", hasher.finalize());
        }
        ChecksumType::SHA256 => {
            let mut hasher = Sha256::new();
            io::copy(&mut bytes, &mut hasher)?;
            actual_checksum = format!("{:x}", hasher.finalize());
        }
        ChecksumType::MD5 => {
            let digest: md5::Digest = md5::compute(&mut bytes);
            actual_checksum = format!("{:x}", digest);
        }
    }

    if actual_checksum != checksum {
        return Ok(false);
    };

    Ok(true)
}

// Extract

pub async fn extract_zip(
    path: PathBuf,
    reader: &mut Cursor<Vec<u8>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut archive: ZipArchive<&mut Cursor<Vec<u8>>> = ZipArchive::new(reader).unwrap();

    for i in 0..archive.len() {
        let mut file: ZipFile = archive.by_index(i)?;
        let outpath: PathBuf = path.join(file.mangled_name());

        if (&*file.name()).starts_with("META-INF/") {
            // Skip extracting the file if it's inside META-INF/
            continue;
        }

        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile: File = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

pub async fn extract_file(
    source: &mut Cursor<Vec<u8>>,
    filename: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut archive: ZipArchive<&mut Cursor<Vec<u8>>> = ZipArchive::new(source).unwrap();
    for i in 0..archive.len() {
        let mut file: ZipFile = archive.by_index(i)?;

        if (&*file.name()).contains(filename) {
            let mut json_content = Vec::new();
            file.read_to_end(&mut json_content)?;
            return Ok(json_content);
        }
    }

    return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "The file to extract was not found.",
    )));
}

// Delete

pub fn delete(path: &str) {
    let path: PathBuf = env::current_exe().unwrap().parent().unwrap().join(path);

    if !path.exists() {
        return;
    }

    fs::remove_file(path).unwrap();
}

// Merge Zips/Jars

pub async fn merge_zips(
    target_zip: &mut Vec<u8>,
    source_zip: &Vec<u8>,
    exclude_meta_inf: bool,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut merged_zip: ZipWriter<Cursor<Vec<u8>>> = ZipWriter::new(Cursor::new(Vec::new()));

    let mut source_zip: ZipArchive<Cursor<&Vec<u8>>> = ZipArchive::new(Cursor::new(source_zip))?;

    for i in 0..source_zip.len() {
        let mut file: ZipFile = source_zip.by_index(i)?;

        if exclude_meta_inf && (&*file.name()).starts_with("META-INF/") {
            // Skip extracting the file if it's inside META-INF/
            continue;
        }
        let options: FileOptions =
            FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        merged_zip.start_file(file.name(), options)?;
        io::copy(&mut file, &mut merged_zip)?;
    }

    let mut target_zip: ZipArchive<Cursor<&mut Vec<u8>>> =
        ZipArchive::new(Cursor::new(target_zip))?;

    for i in 0..target_zip.len() {
        let mut file: ZipFile = target_zip.by_index(i)?;

        if exclude_meta_inf && (&*file.name()).starts_with("META-INF/") {
            // Skip extracting the file if it's inside META-INF/
            continue;
        }
        let options: FileOptions =
            FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        let name: String = file.name().to_owned();
        if let Ok(_) = source_zip.by_name(&name) {
            continue;
        }
        merged_zip.start_file(file.name(), options)?;
        io::copy(&mut file, &mut merged_zip)?;
    }

    let result: Cursor<Vec<u8>> = merged_zip.finish()?;
    Ok(result.into_inner().to_vec())
}

pub fn library_name_to_path(name: &str) -> String {
    let libraries_path: String = check_directory_sync("libraries")
        .to_str()
        .unwrap()
        .to_string();
    let mut final_string: String = libraries_path.clone() + "\\";

    let extension_split: Vec<&str> = name.split("@").collect();

    let tokens: Vec<&str> = extension_split[0].split(":").collect();
    let mut extension: String = ".jar".to_string();

    if 1 < extension_split.len() {
        extension = ".".to_string() + extension_split[1];
    }

    final_string += &tokens[0].replace(".", "\\");
    final_string += "\\";
    final_string += tokens[1];
    final_string += "\\";
    final_string += tokens[2];
    final_string += "\\";
    final_string += tokens[1];
    final_string += "-";
    final_string += tokens[2];

    if 3 < tokens.len() {
        final_string += "-";
        final_string += tokens[3];
    }

    final_string += &extension;
    final_string
}
