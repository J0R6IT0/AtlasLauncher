use reqwest;
use serde_json::{self, Value};
use std::{env, path::PathBuf};

use crate::{common::utils::download_file, utils::directory_checker::check_directory};

pub async fn download(version: u8) -> Result<(), Box<dyn std::error::Error>> {
    if !check_java(version).await {
        return Ok(());
    }
    let path: PathBuf = check_directory(&format!("java/{version}")).await;

    let binary: Value = get_version_info(version).await.unwrap();

    let checksum: &str = binary["package"]["checksum"].as_str().unwrap();
    let link: &str = binary["package"]["link"].as_str().unwrap();

    // Download the file
    download_file::download_file(link, checksum, 56, path.to_str().unwrap(), true)
        .await
        .unwrap();

    Ok(())
}

async fn get_version_info(version: u8) -> Result<Value, Box<dyn std::error::Error>> {
    let arch: &str = match env::consts::ARCH {
        "x86" => "x32",
        "x86_64" => "x64",
        other => other,
    };
    let os: &str = match env::consts::OS {
        "macos" => "mac",
        _ => env::consts::OS,
    };

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
    let path: PathBuf = check_directory(&format!("java/{version}")).await;

    path.read_dir().unwrap().next().is_none()
}
