use crate::utils::check_directory::check_directory;
use reqwest;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::{
    env,
    fs::File,
    io::{self, Cursor},
    path::PathBuf,
};

pub async fn download_file(
    url: &str,
    checksum: &str,
    checksum_type: u8,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if file exists

    let exe_path: PathBuf = env::current_exe().unwrap();

    let file_path: PathBuf = exe_path.parent().unwrap().join(path);

    let mut path = file_path.clone();
    path.pop();

    if file_path.is_file() {
        return Ok(());
    }

    // Download the file

    let mut retry_count = 0;
    let bytes;

    loop {
        let response = match reqwest::get(url).await {
            Ok(response) => response,
            Err(error) => {
                if error.is_connect() || error.is_timeout() {
                    // Connection timed out error, retry
                    if retry_count >= 3 {
                        return Err(Box::new(error)); // Maximum number of retries reached
                    }
                    retry_count += 1;
                    continue;
                } else {
                    return Err(Box::new(error));
                }
            }
        };

        bytes = response.bytes().await?;
        break;
    }

    let mut reader = Cursor::new(&bytes);

    // Verify the checksum

    let actual_checksum: String;

    if checksum_type == 1 {
        let mut hasher = Sha1::new();
        io::copy(&mut reader, &mut hasher)?;
        actual_checksum = format!("{:x}", hasher.finalize());
    } else {
        let mut hasher = Sha256::new();
        io::copy(&mut reader, &mut hasher)?;
        actual_checksum = format!("{:x}", hasher.finalize());
    }

    if actual_checksum != checksum {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Checksum missmatch.",
        )));
    }

    // Save the file

    check_directory(path.to_str().unwrap()).await;

    let mut file = File::create(file_path)?;
    reader.set_position(0);
    io::copy(&mut reader, &mut file)?;

    Ok(())
}
