use serde::{Deserialize, Serialize};
use std::{env, fs, io::Write, path};

use tokio::task;

#[derive(Serialize, Deserialize)]
struct VersionInfo {
    id: String,
    url: String,
}

pub async fn download(version_type: &str, version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = get_version_url(version_type, version).await?;

    let response: reqwest::Response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to download json: {}", e))?;

    let text: String = response
        .text()
        .await
        .map_err(|e| format!("Failed to convert response to text {}", e))?;

    let json: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| format!("Failed to convert text to json {}", e))?;

    let client_url: &str = json["downloads"]["client"]["url"]
        .as_str()
        .unwrap_or_default();
    download_client(client_url, version).await;

    let assets_url: &str = json["assetIndex"]["url"].as_str().unwrap_or_default();
    download_assets(&assets_url).await;

    let libraries: &Vec<serde_json::Value> = json["libraries"].as_array().unwrap();
    download_libraries(&libraries).await;
    println!("Done downloading game files");

    Ok(())
}

async fn get_version_url(
    version_type: &str,
    version: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let exe_path: path::PathBuf = env::current_exe().unwrap();

    let file_path: path::PathBuf = exe_path
        .parent()
        .ok_or_else(|| "Could not get parent directory".to_string())?
        .join(String::from("launcher/version-info/") + version_type + ".json");

    let content = fs::read_to_string(file_path).unwrap();
    let versions: Vec<VersionInfo> = serde_json::from_str(&content).unwrap();

    if let Some(v) = versions.iter().find(|v| v.id == version) {
        return Ok(v.url.clone());
    }

    Err("Version not found".into())
}

async fn download_client(url: &str, version: &str) -> Result<(), String> {
    let exe_path: path::PathBuf = env::current_exe().unwrap();

    let path = format!("versions/{version}/{version}.jar");

    let file_path: path::PathBuf = exe_path.parent().unwrap().join(path);

    if file_path.exists() {
        return Ok(());
    } else {
        fs::create_dir_all(
            exe_path
                .parent()
                .unwrap()
                .join(format!("versions/{version}")),
        )
        .unwrap();
    }

    println!("{:?}", file_path);
    let bytes = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to download object: {}", e))?
        .bytes()
        .await
        .map_err(|e| format!("Failed to read object bytes: {}", e))?;

    let mut file =
        fs::File::create(file_path).map_err(|e| format!("Failed to create file: {}", e))?;

    file.write_all(&bytes)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

async fn download_assets(url: &str) -> Result<(), String> {
    let response: serde_json::Value = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to download json: {}", e))?
        .json()
        .await
        .unwrap();

    let objects = response
        .get("objects")
        .unwrap()
        .as_object()
        .unwrap()
        .to_owned();

    let mut download_tasks = vec![];

    let exe_path: path::PathBuf = env::current_exe().unwrap();
    for (_, object) in objects.clone() {
        if let Some(hash) = object.get("hash").and_then(|hash| hash.as_str()) {
            let folder_path: path::PathBuf = exe_path
                .parent()
                .unwrap()
                .join(String::from("assets/objects/") + &hash[0..2]);
            if !folder_path.exists() {
                fs::create_dir_all(folder_path);
            }

            let asset_url = format!(
                "https://resources.download.minecraft.net/{}/{}",
                &hash[0..2],
                hash
            );
            let download_task = task::spawn(download_object(asset_url, hash.to_owned()));
            download_tasks.push(download_task);
        }
    }

    for download_task in download_tasks {
        download_task
            .await
            .map_err(|e| format!("Failed to download object: {}", e))??;
    }

    Ok(())
}

async fn download_object(url: String, hash: String) -> Result<(), String> {
    let exe_path: path::PathBuf = env::current_exe().unwrap();

    let path = format!("assets/objects/{}/{}", &hash[0..2], hash);

    let file_path: path::PathBuf = exe_path.parent().unwrap().join(path);

    if file_path.exists() {
        return Ok(());
    }
    let bytes = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to download object: {}", e))?
        .bytes()
        .await
        .map_err(|e| format!("Failed to read object bytes: {}", e))?;

    let mut file =
        fs::File::create(file_path).map_err(|e| format!("Failed to create file: {}", e))?;

    file.write_all(&bytes)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

async fn download_libraries(libraries: &Vec<serde_json::Value>) -> Result<(), String> {
    let mut download_tasks = vec![];

    for download in libraries.iter() {
        let mut must_download: bool = true;
        if let Some(rules) = download.get("rules") {
            for rule in rules.as_array().unwrap().iter() {
                if let Some(action) = rule.get("action").and_then(|a| a.as_str()) {
                    if action == "allow" {
                        if let Some(os) = rule
                            .get("os")
                            .and_then(|os| os.get("name").and_then(|n| n.as_str()))
                        {
                            if os != "windows" {
                                must_download = false;
                            }
                        }
                    }
                }
            }
        }

        let url = download["downloads"]["artifact"]["url"]
            .as_str()
            .unwrap_or_default();
        let path = download["downloads"]["artifact"]["path"]
            .as_str()
            .unwrap_or_default();

        if must_download {
            let download_task = task::spawn(download_library(url.to_owned(), path.to_owned()));
            download_tasks.push(download_task);
        }
    }

    for download_task in download_tasks {
        download_task
            .await
            .map_err(|e| format!("Failed to download object: {}", e))??;
    }

    Ok(())
}

async fn download_library(url: String, path: String) -> Result<(), String> {
    let exe_path: path::PathBuf = env::current_exe().unwrap();

    let file_path: path::PathBuf = exe_path.parent().unwrap().join(format!("libraries/{}", &path));
    let mut folder_path = file_path.clone();
    folder_path.pop();

    if file_path.exists() {
        return Ok(());
    } else {
        fs::create_dir_all(&folder_path).unwrap();
    }

    let bytes = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to download library: {}", e))?
        .bytes()
        .await
        .map_err(|e| format!("Failed to read library bytes: {}", e))?;

    let mut file =
        fs::File::create(file_path).map_err(|e| format!("Failed to create file: {}", e))?;

    file.write_all(&bytes)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}
