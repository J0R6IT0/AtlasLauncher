use serde_json::Value;
use tauri::{AppHandle, Manager};

use crate::{
    common::{
        minecraft::downloader::download_libraries,
        utils::{
            directory::check_directory,
            file::{download_as_json, read_as_value, ChecksumType},
        },
    },
    data::models::{BaseEventPayload, DownloadInstanceEventPayload},
};

pub async fn download_manifest(id: &str, quilt: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let quilt: String = quilt.replace("quilt-", "");

    let exists: std::path::PathBuf = check_directory("launcher/meta/org.quiltmc")
        .await
        .join(format!("{quilt}-{id}.json"));

    if exists.exists() {
        return read_as_value(&format!("launcher/meta/org.quiltmc/{quilt}-{id}.json")).await;
    }

    let manifest = download_as_json(
        &format!("https://meta.quiltmc.org/v3/versions/loader/{id}/{quilt}/profile/json"),
        "",
        &ChecksumType::SHA1,
        exists.to_str().unwrap(),
        false,
        false,
        None,
    )
    .await?;

    Ok(manifest)
}

pub async fn download_quilt(
    id: &str,
    quilt: &str,
    app: &AppHandle,
    instance_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let quilt_version_manifest: Value =
        read_as_value(&format!("launcher/meta/org.quiltmc/{quilt}-{id}.json")).await?;

    app.emit_all(
        "download",
        DownloadInstanceEventPayload {
            base: BaseEventPayload {
                message: String::from("Downloading Quilt libraries"),
                status: String::from("Loading"),
            },
            total: 0,
            downloaded: 0,
            name: instance_name.to_string(),
        },
    )
    .unwrap();

    if let Some(libraries) = quilt_version_manifest["libraries"].as_array() {
        let id_copy: String = id.to_string();
        let libraries_copy: Vec<Value> = libraries.clone();
        let handle_copy: AppHandle = app.clone();
        let instance_name_copy: String = instance_name.to_string();
        tauri::async_runtime::spawn(async move {
            match download_libraries(
                &libraries_copy,
                &id_copy,
                false,
                &handle_copy,
                &instance_name_copy,
            )
            .await
            {
                Ok(_) => Ok(()),
                Err(err) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("{err}"),
                    ))
                        as Box<dyn std::error::Error + Send + Sync>);
                }
            }
        })
        .await?
        .unwrap();
    }

    Ok(())
}
