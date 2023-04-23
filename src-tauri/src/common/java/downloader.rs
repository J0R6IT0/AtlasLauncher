use serde_json::{self, Value};
use std::{env, path::PathBuf};

use crate::{common::utils::file, utils::directory::check_directory};

use super::get_java_path::get_java_path;

pub async fn download(version: u8) -> Result<(), Box<dyn std::error::Error>> {
    let java_path: String = get_java_path(version).await;
    if !java_path.is_empty() {
        return Ok(());
    }
    let path: PathBuf = check_directory(&format!("java/{version}")).await;

    let binary: Value = get_version_info(version).await.unwrap();

    let checksum: &str = binary["package"]["checksum"].as_str().unwrap();
    let link: &str = binary["package"]["link"].as_str().unwrap();

    // Download the file
    file::download_as_vec(
        link,
        checksum,
        &file::ChecksumType::Sha256,
        path.to_str().unwrap(),
        true,
        false,
    )
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

    let json: Value = file::download_as_json(&format!(
        "https://api.adoptium.net/v3/assets/feature_releases/{version}/ga?os={os}&architecture={arch}&image_type=jre"
    ), "", &file::ChecksumType::Sha1, "", false, false).await?;

    let binaries: &Vec<Value> = json.as_array().unwrap()[0]["binaries"].as_array().unwrap();

    let binary: Value = binaries[0].clone();

    Ok(binary)
}
