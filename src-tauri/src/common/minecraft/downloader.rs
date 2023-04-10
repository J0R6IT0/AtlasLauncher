use serde::{Deserialize, Serialize};
use std::{env, fs, path};

use crate::utils::{download_file::download_file, json_to_file};
use serde_json::{self, Value};

#[derive(Serialize, Deserialize)]
struct VersionInfo {
    id: String,
    url: String,
}

pub async fn download(version_type: &str, version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = get_version_url(version_type, version).await?;

    let response = reqwest::get(url).await?.text().await?;

    json_to_file::save(
        response.as_str(),
        format!("versions/{version}/{version}.json").as_str(),
    );

    let json: Value = serde_json::from_str(response.as_str()).unwrap();

    // client.jar
    let client_url: &str = json["downloads"]["client"]["url"]
        .as_str()
        .unwrap_or_default();

    let client_checksum: &str = json["downloads"]["client"]["sha1"]
        .as_str()
        .unwrap_or_default();

    download_file(
        client_url,
        client_checksum,
        1,
        format!("versions/{version}/{version}.jar").as_str(),
    )
    .await
    .unwrap();

    // assets
    let assets_url: &str = json["assetIndex"]["url"].as_str().unwrap_or_default();
    let assets_index: &str = json["assetIndex"]["id"].as_str().unwrap_or_default();
    download_assets(assets_url, assets_index).await?;

    // libraries
    let libraries: &Vec<serde_json::Value> = json["libraries"].as_array().unwrap();
    download_libraries(&libraries).await?;
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

async fn download_assets(url: &str, id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?.text().await?;

    json_to_file::save(
        response.as_str(),
        format!("assets/indexes/{id}.json").as_str(),
    );

    let json: Value = serde_json::from_str(response.as_str()).unwrap();

    let objects = json.get("objects").unwrap().as_object().unwrap().to_owned();

    let mut download_tasks = vec![];

    for (_, object) in objects {
        let download_task = tauri::async_runtime::spawn(async move {
            let object_hash: &str = object.get("hash").unwrap().as_str().unwrap();
            let object_url: String = format!(
                "https://resources.download.minecraft.net/{}/{}",
                &object_hash[0..2],
                &object_hash
            );
            download_file(
                &object_url,
                object_hash,
                1,
                format!("assets/objects/{}/{}", &object_hash[0..2], &object_hash).as_str(),
            )
            .await
            .unwrap();
        });
        download_tasks.push(download_task);
    }

    for download_task in download_tasks {
        download_task.await?;
    }

    Ok(())
}

async fn download_libraries(
    libraries: &Vec<serde_json::Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut download_tasks = vec![];

    for download in libraries.clone() {
        let download_task = tauri::async_runtime::spawn(async move {
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
            let url: &str = download["downloads"]["artifact"]["url"]
                .as_str()
                .unwrap_or_default();
            let hash: &str = download["downloads"]["artifact"]["sha1"]
                .as_str()
                .unwrap_or_default();
            let path: &str = download["downloads"]["artifact"]["path"]
                .as_str()
                .unwrap_or_default();

            if must_download {
                download_file(&url, hash, 1, format!("libraries/{}", path).as_str())
                    .await
                    .unwrap();
            }

            if let Some(natives) = download["downloads"].get("classifiers") {
                if let Some(_windows_natives) = natives.get("natives-windows") {
                    let url: &str = download["downloads"]["classifiers"]["natives-windows"]["url"]
                        .as_str()
                        .unwrap_or_default();
                    let hash: &str = download["downloads"]["classifiers"]["natives-windows"]
                        ["sha1"]
                        .as_str()
                        .unwrap_or_default();
                    let path: &str = download["downloads"]["classifiers"]["natives-windows"]
                        ["path"]
                        .as_str()
                        .unwrap_or_default();

                    download_file(&url, hash, 1, format!("libraries/{}", path).as_str())
                        .await
                        .unwrap();
                }
            }
        });
        download_tasks.push(download_task);
    }

    for download_task in download_tasks {
        download_task.await?;
    }

    Ok(())
}
