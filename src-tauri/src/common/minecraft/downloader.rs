use serde::{Deserialize, Serialize};
use std::{env, fs, path};

use crate::utils::{download_file::download_file, file_to_json, json_to_file};
use serde_json::{self, Value};

#[derive(Serialize, Deserialize)]
struct VersionInfo {
    id: String,
    url: String,
}

pub async fn download(version_type: &str, version: &str, instance_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut json: Option<Value> = None;

    match file_to_json::read(format!("versions/{version}/{version}.json").as_str()) {
        Ok(file_json) => json = Some(file_json),
        Err(_) => {}
    };

    if json.is_none() {
        let url = get_version_url(version_type, version).await?;

        let response = reqwest::get(url).await?.text().await?;

        json_to_file::save(
            response.as_str(),
            format!("versions/{version}/{version}.json").as_str(),
        );

        json = Some(serde_json::from_str(response.as_str()).unwrap());
    }

    // client.jar

    if let Some(json) = json {
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
            false
        )
        .await
        .unwrap();

        // assets
        let assets_url: &str = json["assetIndex"]["url"].as_str().unwrap_or_default();
        let assets_index: &str = json["assetIndex"]["id"].as_str().unwrap_or_default();
        download_assets(assets_url, assets_index, &version, instance_name).await?;

        // libraries
        let libraries: &Vec<serde_json::Value> = json["libraries"].as_array().unwrap();
        download_libraries(&libraries, &version).await?;
    }

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

async fn download_assets(url: &str, id: &str, version: &str, instance_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut json: Option<Value> = None;

    match file_to_json::read(format!("assets/indexes/{id}.json").as_str()) {
        Ok(file_json) => json = Some(file_json),
        Err(_) => {}
    };

    if json.is_none() {
        let response = reqwest::get(url).await?.text().await?;

        json_to_file::save(
            response.as_str(),
            format!("assets/indexes/{id}.json").as_str(),
        );

        json = Some(serde_json::from_str(response.as_str()).unwrap());
    }

    if let Some(json) = json {
        let objects = json.get("objects").unwrap().as_object().unwrap().to_owned();

        let mut download_tasks = vec![];

        for (key, object) in objects {
            let mc_version: String = String::from(version);
            let mc_instance: String = String::from(instance_name);
            let assets_id = String::from(id);
            let download_task = tauri::async_runtime::spawn(async move {
                let object_hash: &str = object.get("hash").unwrap().as_str().unwrap();
                let object_url: String = format!(
                    "https://resources.download.minecraft.net/{}/{}",
                    &object_hash[0..2],
                    &object_hash
                );

                let path: String;
                if assets_id == "legacy" || assets_id == "pre-1.6" {
                    path = format!("assets/virtual/legacy/{}", key);
                    
                }
                else {
                    path = format!("assets/objects/{}/{}" , &object_hash[0..2], &object_hash);
                }
                    download_file(
                        &object_url,
                        object_hash,
                        1,
                        &path,
                        false
                    )
                    .await
                    .unwrap();

                if assets_id == "pre-1.6" && (mc_version == "1.0" || mc_version == "1.1" || mc_version.starts_with("b") || mc_version.starts_with("a") || mc_version.starts_with("inf") || mc_version.starts_with("c0") || mc_version.starts_with("rd-")) {
                    download_file(
                        &object_url,
                        object_hash,
                        1,
                        format!("instances/{}/resources/{}" , mc_instance, key).as_str(),
                        false
                    )
                    .await
                    .unwrap();
                }

            });
            download_tasks.push(download_task);
        }

        for download_task in download_tasks {
            download_task.await?;
        }
    }
    Ok(())
}

async fn download_libraries(
    libraries: &Vec<serde_json::Value>,
    version: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut download_tasks = vec![];

    for download in libraries.clone() {
        let mc_version: String = String::from(version);
        let download_task = tauri::async_runtime::spawn(async move {
            if let Some(_artifact) = download["downloads"].get("artifact") {
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
                    download_file(&url, hash, 1, format!("libraries/{}", path).as_str(), false)
                        .await
                        .unwrap();
                }
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

                    download_file(&url, hash, 1, format!("natives/{mc_version}").as_str(), true)
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
