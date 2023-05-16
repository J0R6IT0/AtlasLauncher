use std::env::consts::{ARCH, OS};

use futures::{future::join_all, stream, StreamExt};
use serde::{Deserialize, Serialize};
use tauri::{async_runtime, AppHandle, Manager};

use crate::{
    common::utils::{
        file::{library_name_to_raw_path, write_vec},
        log::write_line,
    },
    data::models::{BaseEventPayload, DownloadInstanceEventPayload},
    utils::file,
};
use serde_json::{self, Map, Value};

use super::versions::get_version;

#[derive(Serialize, Deserialize)]
struct VersionInfo {
    id: String,
    url: String,
}

pub async fn download(
    id: &str,
    app: &AppHandle,
    instance_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let version: crate::data::models::MinecraftVersionData = get_version(id).await?;
    let url: String = version.url;

    let json: Value = file::download_as_json(
        &url,
        "",
        &file::ChecksumType::SHA1,
        format!("launcher/meta/net.minecraft/{id}.json").as_str(),
        false,
        false,
        None,
    )
    .await?;

    let mut download_tasks: Vec<async_runtime::JoinHandle<()>> = vec![];

    let total_size: u64 = compute_total_size(&json);

    app.emit_all(
        "download",
        DownloadInstanceEventPayload {
            base: BaseEventPayload {
                message: format!("Downloading game files"),
                status: String::from("Loading"),
            },
            total: total_size,
            downloaded: 0,
            name: instance_name.to_string(),
        },
    )?;

    // client.jar
    let json_copy: Value = json.clone();
    let version_copy: String = id.clone().to_owned();
    let instance_name_copy: String = instance_name.to_string();
    let handle_copy: AppHandle = app.clone();
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
            &file::ChecksumType::SHA1,
            format!(
                "versions/{}.jar",
                if version_copy.starts_with("_") {
                    version_copy[1..].to_string()
                } else {
                    version_copy
                }
            )
            .as_str(),
            false,
            false,
            Some((&handle_copy, &instance_name_copy)),
        )
        .await
        .unwrap();
    });
    download_tasks.push(download_task);

    // assets

    if json.get("assetIndex").is_some() {
        let json_copy: Value = json.clone();
        let handle_copy: AppHandle = app.clone();
        let instance_name_copy: String = instance_name.to_string();
        let download_task: async_runtime::JoinHandle<()> =
            tauri::async_runtime::spawn(async move {
                let assets_url: &str = json_copy["assetIndex"]["url"].as_str().unwrap_or_default();
                let assets_index: &str = json_copy["assetIndex"]["id"].as_str().unwrap_or_default();
                download_assets(assets_url, assets_index, &handle_copy, &instance_name_copy)
                    .await
                    .unwrap();
            });
        download_tasks.push(download_task);
    }

    // logging
    let json_copy: Value = json.clone();
    let handle_copy: AppHandle = app.clone();
    let instance_name_copy: String = instance_name.to_string();

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
                        &file::ChecksumType::SHA1,
                        format!("assets/log_configs/{id}").as_str(),
                        false,
                        false,
                        Some((&handle_copy, &instance_name_copy)),
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
    let libraries_arg: String =
        download_libraries(&libraries, &id, false, app, &instance_name).await?;

    join_all(download_tasks).await;
    return Ok(libraries_arg);
}

fn compute_total_size(version_info: &Value) -> u64 {
    let mut total_size: u64 = 0;
    total_size += version_info["downloads"]["client"]["size"]
        .as_u64()
        .unwrap_or(0);
    total_size += version_info["assetIndex"]["totalSize"]
        .as_u64()
        .unwrap_or(0);
    total_size += version_info["logging"]["client"]["file"]["size"]
        .as_u64()
        .unwrap_or(0);

    for download in version_info["libraries"].as_array().unwrap().clone() {
        let mut must_download: bool = true;
        let formatted_os: &str = match OS {
            "macos" => "osx",
            _ => OS,
        };
        if let Some(rules) = download.get("rules") {
            for rule in rules.as_array().unwrap().iter() {
                if let Some(action) = rule.get("action").and_then(|a| a.as_str()) {
                    if action == "allow" {
                        if let Some(os) = rule.get("os") {
                            if let Some(name) = os.get("name") {
                                if name.as_str().unwrap() != formatted_os {
                                    must_download = false;
                                }
                            }
                        }
                    } else if action == "disallow" {
                        if let Some(os) = rule.get("os") {
                            if let Some(name) = os.get("name") {
                                if name.as_str().unwrap() == formatted_os {
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
                total_size += artifact["size"].as_u64().unwrap_or(0);
            }
            if let Some(natives_classifier) = download["natives"].get(formatted_os) {
                let arch: &str = match ARCH {
                    "x86" => "32",
                    "x86_64" => "64",
                    _ => "64",
                };
                let natives_classifier: String = natives_classifier
                    .as_str()
                    .unwrap()
                    .replace("${arch}", arch);

                if let Some(natives) = download["downloads"]["classifiers"].get(natives_classifier)
                {
                    total_size += natives["size"].as_u64().unwrap_or(0);
                }
            }
        }
    }
    total_size
}

async fn download_assets(
    url: &str,
    id: &str,
    app: &AppHandle,
    instance_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let json: Value = file::download_as_json(
        url,
        "",
        &file::ChecksumType::SHA1,
        format!("assets/indexes/{id}.json").as_str(),
        false,
        false,
        None,
    )
    .await?;

    let objects: Map<String, Value> = json.get("objects").unwrap().as_object().unwrap().to_owned();

    let download_tasks = stream::iter(objects.into_iter().map(|object| async {
        let id_copy: String = id.to_string();
        let handle_copy = app.clone();
        let instance_name_copy: String = instance_name.to_string();
        async_runtime::spawn(async move {
            let object_hash: &str = object.1.get("hash").unwrap().as_str().unwrap();
            let custom_url: Option<&str> = object.1["custom_url"].as_str();

            let object_url: String;

            if custom_url.is_some() {
                object_url = custom_url.unwrap().to_string();
            } else {
                object_url = format!(
                    "https://resources.download.minecraft.net/{}/{}",
                    &object_hash[0..2],
                    &object_hash
                );
            }

            let path: String = format!("assets/objects/{}/{}", &object_hash[0..2], &object_hash);

            let bytes = match file::download_as_vec(
                &object_url,
                object_hash,
                &file::ChecksumType::SHA1,
                &path,
                false,
                false,
                Some((&handle_copy, &instance_name_copy)),
            )
            .await
            {
                Ok(vec) => vec,
                Err(e) => {
                    write_line(&e.to_string());
                    return;
                }
            };

            if id_copy == "legacy" || id_copy == "1.7.10" {
                write_vec(&bytes, &format!("assets/virtual/legacy/{}", object.0)).unwrap();
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

pub async fn download_libraries(
    libraries: &Vec<serde_json::Value>,
    version: &str,
    skip_natives: bool,
    app: &AppHandle,
    instance_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut libraries_arg: String = String::from("");

    let mut download_tasks: Vec<async_runtime::JoinHandle<()>> = vec![];

    for download in libraries.clone() {
        let mc_version: String = if version.starts_with("_") {
            version[1..].to_string()
        } else {
            String::from(version)
        };
        let mut must_download: bool = true;
        let formatted_os: &str = match OS {
            "macos" => "osx",
            _ => OS,
        };
        if let Some(rules) = download.get("rules") {
            for rule in rules.as_array().unwrap().iter() {
                if let Some(action) = rule.get("action").and_then(|a| a.as_str()) {
                    if action == "allow" {
                        if let Some(os) = rule.get("os") {
                            if let Some(name) = os.get("name") {
                                if name.as_str().unwrap() != formatted_os {
                                    must_download = false;
                                }
                            }
                        }
                    } else if action == "disallow" {
                        if let Some(os) = rule.get("os") {
                            if let Some(name) = os.get("name") {
                                if name.as_str().unwrap() == formatted_os {
                                    must_download = false;
                                }
                            }
                        }
                    }
                }
            }
        }
        if must_download {
            let handle_copy: AppHandle = app.clone();
            let instance_name_copy: String = instance_name.to_string();
            if let Some(artifact) = download["downloads"].get("artifact") {
                let artifact: Value = artifact.to_owned();
                let library_path: &str = artifact["path"].as_str().unwrap_or_default();
                libraries_arg = format!("{libraries_arg}${{libraries_path}}/{library_path};",);
                let download_task: async_runtime::JoinHandle<()> =
                    tauri::async_runtime::spawn(async move {
                        let mut url: String =
                            artifact["url"].as_str().unwrap_or_default().to_string();
                        let hash: &str = artifact["sha1"].as_str().unwrap_or_default();
                        let library_path: &str = artifact["path"].as_str().unwrap_or_default();
                        if url.is_empty() {
                            if library_path.starts_with("net/minecraftforge/forge") {
                                if library_path.contains("universal") {
                                    url =
                                        format!("https://maven.minecraftforge.net/{library_path}");
                                } else {
                                    url = format!(
                                        "https://maven.minecraftforge.net/{}-launcher.jar",
                                        library_path.replace(".jar", "")
                                    );
                                }
                            }
                        }
                        file::download_as_vec(
                            &url,
                            &hash,
                            &file::ChecksumType::SHA1,
                            format!("libraries/{}", &library_path).as_str(),
                            false,
                            false,
                            Some((&handle_copy, &instance_name_copy)),
                        )
                        .await
                        .unwrap();
                    });
                download_tasks.push(download_task);
            }
            let instance_name_copy: String = instance_name.to_string();
            if let Some(natives_classifier) = download["natives"].get(formatted_os) {
                if skip_natives {
                    continue;
                }
                let arch: &str = match ARCH {
                    "x86" => "32",
                    "x86_64" => "64",
                    _ => "64",
                };
                let natives_classifier: String = natives_classifier
                    .as_str()
                    .unwrap()
                    .replace("${arch}", arch);

                if let Some(natives) = download["downloads"]["classifiers"].get(natives_classifier)
                {
                    let handle_copy = app.clone();

                    let natives: Value = natives.to_owned();

                    let download_task: async_runtime::JoinHandle<()> =
                        tauri::async_runtime::spawn(async move {
                            let url: &str = natives["url"].as_str().unwrap_or_default();
                            let hash: &str = natives["sha1"].as_str().unwrap_or_default();

                            file::download_as_vec(
                                &url,
                                hash,
                                &file::ChecksumType::SHA1,
                                format!("natives/{mc_version}").as_str(),
                                true,
                                false,
                                Some((&handle_copy, &instance_name_copy)),
                            )
                            .await
                            .unwrap();
                        });
                    download_tasks.push(download_task);
                }
            }
            if let Some(url) = download["url"].as_str() {
                let name = library_name_to_raw_path(download["name"].as_str().unwrap());
                let final_url = format!("{url}{name}");
                libraries_arg = format!("{libraries_arg}${{libraries_path}}/{name};",);
                if skip_natives {
                    continue;
                }
                let download_task: async_runtime::JoinHandle<()> =
                    tauri::async_runtime::spawn(async move {
                        file::download_as_vec(
                            &final_url,
                            "",
                            &file::ChecksumType::SHA1,
                            format!("libraries/{}", &name).as_str(),
                            false,
                            false,
                            None,
                        )
                        .await
                        .unwrap();
                    });
                download_tasks.push(download_task);
            }
        }
    }
    join_all(download_tasks).await;

    Ok(libraries_arg)
}
