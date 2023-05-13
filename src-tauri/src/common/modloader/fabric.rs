use serde_json::Value;

use crate::common::{utils::{
    directory::check_directory,
    file::{download_as_json, read_as_value, ChecksumType},
}, minecraft::downloader::download_libraries};

pub async fn download_manifest(
    id: &str,
    fabric: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let fabric: String = fabric.replace("fabric-", "");

    let exists: std::path::PathBuf = check_directory("launcher/meta/net.fabricmc")
        .await
        .join(format!("{fabric}.json"));

    if exists.exists() {
        return read_as_value(&format!("launcher/meta/net.fabricmc/{fabric}.json")).await;
    }

    let manifest = download_as_json(
        &format!("https://meta.fabricmc.net/v2/versions/loader/{id}/{fabric}/profile/json"),
        "",
        &ChecksumType::SHA1,
        exists.to_str().unwrap(),
        false,
        false,
    )
    .await?;

    Ok(manifest)
}

pub async fn download_fabric(id: &str, fabric: &str) -> Result<(), Box<dyn std::error::Error>> {
    let fabric_version_manifest: Value =
        read_as_value(&format!("launcher/meta/net.fabricmc/{fabric}.json")).await?;

    if let Some(libraries) = fabric_version_manifest["libraries"].as_array() {
        let id_copy: String = id.to_string();
        let libraries_copy: Vec<Value> = libraries.clone();
        tauri::async_runtime::spawn(async move {
            match download_libraries(&libraries_copy, &id_copy, false).await {
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
