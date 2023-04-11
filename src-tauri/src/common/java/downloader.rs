use reqwest;
use serde_json::{self, Value};
use sha2::{Digest, Sha256};
use std::{
    env,
    fs::File,
    io::{self, Cursor},
};
use zip::ZipArchive;

use crate::utils::directory_checker::check_directory;

pub async fn download(version: u8) -> Result<(), Box<dyn std::error::Error>> {
    if !check_java(version).await {
        return Ok(());
    }
    let path = check_directory(&format!("java/{version}")).await;

    let binary: Value = get_version_info(version).await.unwrap();

    let checksum: &str = binary["package"]["checksum"].as_str().unwrap();
    let link: &str = binary["package"]["link"].as_str().unwrap();

    // Download the file
    let response = reqwest::get(link).await?.bytes().await?;

    let mut reader = Cursor::new(&response);

    // Verify the checksum
    let mut hasher = Sha256::new();
    io::copy(&mut reader, &mut hasher)?;
    let actual_checksum: String = format!("{:x}", hasher.finalize());
    if actual_checksum != checksum {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Checksum missmatch.",
        )));
    }

    // Unzip contents
    let mut archive = ZipArchive::new(reader).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = path.join(file.mangled_name());

        if (&*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

async fn get_version_info(version: u8) -> Result<Value, Box<dyn std::error::Error>> {
    let os = env::consts::OS;
    let mut arch = env::consts::ARCH;

    if arch == "x86" {
        arch = "x32";
    } else if arch == "x86_64" {
        arch = "x64";
    }

    let response: serde_json::Value = reqwest::get(format!(
        "https://api.adoptium.net/v3/assets/feature_releases/{version}/ga?os={os}&architecture={arch}&image_type=jre"
    ))
    .await?
    .json()
    .await
    .map_err(|e| format!("Failed to get json: {}", e))?;

    let binaries: &Vec<Value> = response.as_array().unwrap()[0]["binaries"]
        .as_array()
        .unwrap();

    let binary: Value = binaries[0].clone();

    Ok(binary)
}

async fn check_java(version: u8) -> bool {
    let path = check_directory(&format!("java/{version}")).await;

    path.read_dir().unwrap().next().is_none()
}
