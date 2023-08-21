use std::{io, path::Path, time::Duration};

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

use super::directory::get_path_or_create;

/// Read a file and return the vector of bytes.
///
/// # Errors
/// This function will return an error if `path` does not exist or if [`tokio`](tokio)'s [`fs::read`](fs::read) fails.
///
/// # Examples
/// ```
/// let bytes = read("path/to/file.foo").await.unwrap();
/// ```
pub async fn read<P>(path: P) -> Result<Vec<u8>, &'static str>
where
    P: AsRef<Path>,
{
    let app_directory = APP_DIRECTORY.get().unwrap();
    let file_path = app_directory.join(path);

    if !file_path.exists() {
        return Err("The path does not exist.");
    }

    fs::read(&file_path)
        .await
        .map_err(|_| "Error reading file.")
}

/// Deserialize an instance of type `T` from a JSON file.
///
/// # Errors
/// This function will return an error if `path` does not exist or if [`tokio`](tokio)'s [`fs::read`](fs::read) fails.
/// It can also fail if the JSON does not match `T`.
///
/// # Examples
/// ```
/// let json: serde::Value = read_to_value("path/to/file.json").await.unwrap();
/// ```
pub async fn read_to_value<P, D>(path: P) -> Result<D, &'static str>
where
    P: AsRef<Path>,
    D: for<'de> Deserialize<'de>,
{
    let bytes: Vec<u8> = read(path).await?;
    serde_json::from_slice(&bytes).map_err(|_| "Error parsing json.")
}

/// Writes a vector of bytes to a file.
///
/// # Errors
/// This function will return an error if the file cannot be created or if the data cannot be written to it. See [`File`](File) for more details.
///
/// # Examples
/// ```
/// write_vec(&bytes_to_write, "path/to/file.foo").await?;
/// ```
pub async fn write_vec<P>(data: &[u8], path: P) -> Result<(), &'static str>
where
    P: AsRef<Path>,
{
    let app_directory = APP_DIRECTORY.get().unwrap();
    let file_path = app_directory.join(path);
    let parent_path = file_path.parent().unwrap();

    get_path_or_create(parent_path).await;

    let mut file = File::create(&file_path)
        .await
        .map_err(|_| "Error creating file.")?;

    file.write_all(data)
        .await
        .map_err(|_| "Error writing file.")
}

/// Writes serializable data to a file.
///
/// # Errors
/// This function will return an error if the file cannot be created or if the data cannot be written to it. See [`File`](File) for more details.
///
/// # Examples
/// ```
/// write_value(&some_serializable_data, "path/to/file.json").await?;
/// ```
pub async fn write_value<S, P>(data: &S, path: P) -> Result<(), &'static str>
where
    S: Serialize,
    P: AsRef<Path>,
{
    let bytes: Vec<u8> = serde_json::to_vec(&data).unwrap();
    write_vec(&bytes, path).await?;

    Ok(())
}

// download file

pub async fn download_as_vec<S: AsRef<Path>>(
    url: &str,
    checksum: &str,
    checksum_type: ChecksumType,
    path: S,
    force: bool,
) -> Result<Vec<u8>, &'static str> {
    if !force {
        let mut vec: Option<Vec<u8>> = None;

        if let Ok(file_vec) = read("").await {
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

    if !path.as_ref().is_file() {
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
