use futures::{future::join_all, stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::env;
use tauri::async_runtime;

use crate::utils::file;
use serde_json::{self, Map, Value};

use super::versions::get_version;

#[derive(Serialize, Deserialize)]
struct VersionInfo {
    id: String,
    url: String,
}

pub async fn download(id: &str, instance_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let version: crate::data::models::MinecraftVersionData = get_version(id).await?;
    let url: String = version.url;

    let json: Value = file::download_as_json(
        &url,
        "",
        &file::ChecksumType::Sha1,
        format!("versions/{id}/{id}.json").as_str(),
        false,
    )
    .await?;

    let mut download_tasks: Vec<async_runtime::JoinHandle<()>> = vec![];

    // client.jar
    let json_copy: Value = json.clone();
    let version_copy: String = id.clone().to_owned();
    let download_task: async_runtime::JoinHandle<()> = tauri::async_runtime::spawn(async move {
        let client_url: &str = json_copy["downloads"]["client"]["url"]
            .as_str()
            .unwrap_or_default();

        let client_checksum: &str = json_copy["downloads"]["client"]["sha1"]
            .as_str()
            .unwrap_or_default();

        file::download_as_vec(
            client_url,
            client_checksum,
            &file::ChecksumType::Sha1,
            format!("versions/{version_copy}/{version_copy}.jar").as_str(),
            false,
        )
        .await
        .unwrap();
    });
    download_tasks.push(download_task);

    // assets
    let json_copy: Value = json.clone();
    let instance_name_copy: String = instance_name.clone().to_owned();
    let download_task: async_runtime::JoinHandle<()> = tauri::async_runtime::spawn(async move {
        let assets_url: &str = json_copy["assetIndex"]["url"].as_str().unwrap_or_default();
        let assets_index: &str = json_copy["assetIndex"]["id"].as_str().unwrap_or_default();
        download_assets(assets_url, assets_index, &instance_name_copy)
            .await
            .unwrap();
    });
    download_tasks.push(download_task);

    // logging
    let json_copy: Value = json.clone();
    let download_task: async_runtime::JoinHandle<()> = tauri::async_runtime::spawn(async move {
        if let Some(logging) = json_copy.get("logging") {
            if let Some(client) = logging.get("client") {
                if let Some(file) = client.get("file") {
                    let url: &str = file["url"].as_str().unwrap();
                    let sha1: &str = file["sha1"].as_str().unwrap();
                    let id: &str = file["id"].as_str().unwrap();

                    file::download_as_vec(
                        url,
                        sha1,
                        &file::ChecksumType::Sha1,
                        format!("assets/log_configs/{id}").as_str(),
                        false,
                    )
                    .await
                    .unwrap();
                }
            }
        }
    });
    download_tasks.push(download_task);

    // libraries
    let libraries: &Vec<serde_json::Value> = json["libraries"].as_array().unwrap();
    let libraries_arg: String = download_libraries(&libraries, &id).await?;

    join_all(download_tasks).await;
    return Ok(libraries_arg);
}

async fn download_assets(
    url: &str,
    id: &str,
    instance_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let json: Value = file::download_as_json(
        url,
        "",
        &file::ChecksumType::Sha1,
        format!("assets/indexes/{id}.json").as_str(),
        false,
    )
    .await?;

    let objects: Map<String, Value> = json.get("objects").unwrap().as_object().unwrap().to_owned();

    let download_tasks = stream::iter(objects.into_iter().map(|object| async {
        let id_copy: String = id.clone().to_owned();
        let instance_name_copy: String = instance_name.clone().to_owned();
        async_runtime::spawn(async move {
            let mc_instance: String = String::from(instance_name_copy);
            let assets_id: String = String::from(id_copy);

            let object_hash: &str = object.1.get("hash").unwrap().as_str().unwrap();
            let object_url: String = format!(
                "https://resources.download.minecraft.net/{}/{}",
                &object_hash[0..2],
                &object_hash
            );

            let path: String;
            if assets_id == "legacy" || assets_id == "pre-1.6" {
                path = format!("assets/virtual/legacy/{}", object.0);
            } else {
                path = format!("assets/objects/{}/{}", &object_hash[0..2], &object_hash);
            }
            let vec: Vec<u8> = file::download_as_vec(
                &object_url,
                object_hash,
                &file::ChecksumType::Sha1,
                &path,
                false,
            )
            .await
            .unwrap();

            if assets_id == "pre-1.6" {
                file::write_vec(
                    &vec,
                    format!("instances/{}/resources/{}", mc_instance, object.0).as_str(),
                )
                .unwrap();
            }
        })
        .await
        .unwrap();
    }))
    .buffer_unordered(50)
    .collect::<Vec<_>>();

    download_tasks.await;

    Ok(())
}

async fn download_libraries(
    libraries: &Vec<serde_json::Value>,
    version: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut libraries_arg: String = String::from("");

    let mut download_tasks = vec![];

    for download in libraries.clone() {
        let mc_version: String = String::from(version);
        let mut must_download: bool = true;

        if let Some(rules) = download.get("rules") {
            for rule in rules.as_array().unwrap().iter() {
                if let Some(action) = rule.get("action").and_then(|a| a.as_str()) {
                    if action == "allow" {
                        if let Some(os) = rule.get("os") {
                            if let Some(name) = os.get("name") {
                                if name.as_str().unwrap() != "windows" {
                                    must_download = false;
                                }
                            }
                        }
                    } else if action == "disallow" {
                        if let Some(os) = rule.get("os") {
                            if let Some(name) = os.get("name") {
                                if name.as_str().unwrap() == "windows" {
                                    must_download = false;
                                }
                            }
                        }
                    }
                }
            }
        }
        if must_download {
            if let Some(artifact) = download["downloads"].get("artifact") {
                let artifact: Value = artifact.to_owned();
                let library_path: &str = artifact["path"].as_str().unwrap_or_default();
                libraries_arg = format!("{libraries_arg}[libraries_path]/{library_path};",);
                let download_task: async_runtime::JoinHandle<()> =
                    tauri::async_runtime::spawn(async move {
                        let url: &str = artifact["url"].as_str().unwrap_or_default();
                        let hash: &str = artifact["sha1"].as_str().unwrap_or_default();
                        let library_path: &str = artifact["path"].as_str().unwrap_or_default();
                        file::download_as_vec(
                            &url,
                            &hash,
                            &file::ChecksumType::Sha1,
                            format!("libraries/{}", &library_path).as_str(),
                            false,
                        )
                        .await
                        .unwrap();
                    });
                download_tasks.push(download_task);
            }
            if let Some(natives) = download["downloads"].get("classifiers") {
                let natives: Value = natives.to_owned();
                let arch: &str = match env::consts::ARCH {
                    "x86" => "32",
                    "x86_64" => "64",
                    other => other,
                };
                if let Some(windows_natives) = natives
                    .get("natives-windows")
                    .or_else(|| natives.get("natives-windows-".to_string() + arch))
                {
                    let windows_natives: Value = windows_natives.to_owned();

                    let download_task: async_runtime::JoinHandle<()> =
                        tauri::async_runtime::spawn(async move {
                            let url: &str = windows_natives["url"].as_str().unwrap_or_default();
                            let hash: &str = windows_natives["sha1"].as_str().unwrap_or_default();

                            file::download_as_vec(
                                &url,
                                hash,
                                &file::ChecksumType::Sha1,
                                format!("natives/{mc_version}").as_str(),
                                true,
                            )
                            .await
                            .unwrap();
                        });
                    download_tasks.push(download_task);
                }
            }
        }
    }

    for download_task in download_tasks {
        download_task.await?;
    }
    Ok(libraries_arg)
}
