use std::{io, time::Duration};

use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Digest, Sha256};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
    time::sleep,
};

use crate::{models::ChecksumType, APP_DIRECTORY};

use super::directory::check_directory;

// read file

pub async fn read_as_vec<S: AsRef<str>>(path: S) -> Result<Vec<u8>, &'static str> {
    let app_directory = APP_DIRECTORY.get().unwrap();
    let file_path = app_directory.join(path.as_ref());

    if !file_path.exists() {
        return Err("The path does not exist.");
    }

    match fs::read(&file_path).await {
        Ok(contents) => Ok(contents),
        Err(_) => Err("Error reading file."),
    }
}

pub async fn read_as_value<T: for<'de> Deserialize<'de>, S: AsRef<str>>(
    path: S,
) -> Result<T, &'static str> {
    let bytes: Vec<u8> = read_as_vec(path).await?;
    match serde_json::from_slice(&bytes) {
        Ok(result) => Ok(result),
        Err(_) => Err("Error reading json."),
    }
}

// write file

pub async fn write_vec<S: AsRef<str>>(data: &[u8], path: S) -> Result<(), &'static str> {
    let app_directory = APP_DIRECTORY.get().unwrap();

    let file_path = app_directory.join(path.as_ref());
    let parent_path = file_path.parent().unwrap();
    check_directory(parent_path.to_str().unwrap()).await;

    let mut file = match File::create(&file_path).await {
        Ok(file) => file,
        Err(_) => {
            return Err("Error creating file.");
        }
    };

    match file.write_all(data).await {
        Ok(_) => Ok(()),
        Err(_) => Err("Error writing file."),
    }
}

pub async fn write_value<T: Serialize, S: AsRef<str>>(
    data: &T,
    path: S,
) -> Result<(), &'static str> {
    let bytes: Vec<u8> = serde_json::to_vec(&data).unwrap();
    write_vec(&bytes, path).await?;

    Ok(())
}

// download file

pub async fn download_as_vec<S: AsRef<str>>(
    url: &str,
    checksum: &str,
    checksum_type: ChecksumType,
    path: S,
    force: bool,
) -> Result<Vec<u8>, &'static str> {
    if !force {
        let mut vec: Option<Vec<u8>> = None;

        if let Ok(file_vec) = read_as_vec("").await {
            vec = Some(file_vec);
        }

        if vec.is_some() {
            let vec: Vec<u8> = Some(vec).unwrap().unwrap();
            if verify_hash(checksum, checksum_type, &vec).await {
                return Ok(vec);
            }
        }
    }

    let mut retry_count: u8 = 0;
    let bytes: Vec<u8>;

    loop {
        let response = match reqwest::get(url).await {
            Ok(response) => response,
            Err(_) => {
                if retry_count >= 5 {
                    return Err("Error receiving response.");
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
                Err(_) => {
                    if stream_retry_count > 10 {
                        return Err("Error reading file data.");
                    }
                    stream_retry_count += 1;
                    sleep(Duration::from_secs((1 + stream_retry_count).into())).await;
                    continue;
                }
            };
            current_bytes.extend_from_slice(&chunk);
        }

        if verify_hash(checksum, checksum_type, &current_bytes).await {
            bytes = current_bytes;
            break;
        }
        retry_count += 1;
        sleep(Duration::from_secs((1 + retry_count).into())).await;
    }

    if !path.as_ref().is_empty() {
        write_vec(&bytes.to_vec(), path).await?;
    }

    Ok(bytes.to_vec())
}

pub async fn download_as_json<T: for<'de> Deserialize<'de>>(
    url: &str,
    checksum: &str,
    checksum_type: ChecksumType,
    path: &str,
    force: bool,
) -> Result<T, &'static str> {
    let bytes: Vec<u8> = download_as_vec(url, checksum, checksum_type, path, force).await?;
    match serde_json::from_slice(&bytes) {
        Ok(result) => Ok(result),
        Err(_) => Err("Error reading json."),
    }
}

// verify file

pub async fn verify_hash(checksum: &str, checksum_type: ChecksumType, data: &[u8]) -> bool {
    if checksum.is_empty() {
        return true;
    }

    let mut bytes: &[u8] = data;

    let actual_checksum = match checksum_type {
        ChecksumType::SHA1 => {
            let mut hasher = Sha1::new();
            match io::copy(&mut bytes, &mut hasher) {
                Ok(_) => {}
                Err(_) => {
                    return false;
                }
            }
            format!("{:x}", hasher.finalize())
        }
        ChecksumType::SHA256 => {
            let mut hasher = Sha256::new();
            match io::copy(&mut bytes, &mut hasher) {
                Ok(_) => {}
                Err(_) => {
                    return false;
                }
            }
            format!("{:x}", hasher.finalize())
        }
        ChecksumType::MD5 => {
            let digest: md5::Digest = md5::compute(bytes);
            format!("{:x}", digest)
        }
    };

    if actual_checksum != checksum {
        return false;
    }

    true
}
