use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::{
    env,
    fs::{self, File},
    io::{self, Cursor, Write},
    path::Path,
    path::PathBuf,
};
use tokio::time::{sleep, Duration};
use zip::{read::ZipFile, ZipArchive};

use super::directory::{self, check_directory_sync};

pub enum ChecksumType {
    Sha1,
    Sha256,
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
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut vec: Option<Vec<u8>> = None;

    match read_as_vec(path).await {
        Ok(file_vec) => vec = Some(file_vec),
        Err(_) => {}
    }

    if !vec.is_none() {
        let vec: Vec<u8> = Some(vec).unwrap().unwrap();
        if verify_hash(checksum, checksum_type, &vec).await? {
            return Ok(vec);
        }
    }

    let mut retry_count: u8 = 0;
    let mut bytes;

    loop {
        let response: Response = match reqwest::get(url).await {
            Ok(response) => response,
            Err(error) => {
                if retry_count >= 5 {
                    return Err(Box::new(error));
                }
                retry_count += 1;
                sleep(Duration::from_secs((1 + retry_count).into())).await;
                println!("retryting {}", retry_count);
                continue;
            }
        };

        bytes = response.bytes().await?;
        if verify_hash(checksum, checksum_type, &bytes).await? {
            break;
        }
        retry_count += 1;
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
) -> Result<Value, Box<dyn std::error::Error>> {
    let vec: Vec<u8> = download_as_vec(url, checksum, checksum_type, path, extract).await?;
    let json: Value = serde_json::from_slice(&vec)?;
    Ok(json)
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
        ChecksumType::Sha1 => {
            let mut hasher = Sha1::new();
            io::copy(&mut bytes, &mut hasher)?;
            actual_checksum = format!("{:x}", hasher.finalize());
        }
        ChecksumType::Sha256 => {
            let mut hasher = Sha256::new();
            io::copy(&mut bytes, &mut hasher)?;
            actual_checksum = format!("{:x}", hasher.finalize());
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

// Delete

pub fn delete(path: &str) {
    let path: PathBuf = env::current_exe().unwrap().parent().unwrap().join(path);

    if !path.exists() {
        return;
    }

    fs::remove_file(path).unwrap();
}
