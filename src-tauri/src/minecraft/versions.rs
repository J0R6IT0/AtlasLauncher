use serde_json::Value;

use crate::{
    data::{constants::MINECRAFT_VERSION_MANIFEST, enums::ChecksumType},
    util::file::download_as_json,
};

use std::sync::OnceLock;

static VERSIONS: OnceLock<Vec<Value>> = OnceLock::new();

#[tauri::command]
pub async fn update_minecraft_version_manifest() -> Result<Vec<Value>, &'static str> {
    if VERSIONS.get().is_none() {
        let manifest: Value = download_as_json(
            MINECRAFT_VERSION_MANIFEST,
            "",
            ChecksumType::SHA1,
            "launcher/meta/minecraft/version_manifest.json",
            true,
        )
        .await?;

        let versions = manifest["versions"].as_array().unwrap().to_owned();
        match VERSIONS.set(versions.clone()) {
            Ok(_) => {}
            Err(_) => return Err("Error setting OnceLock value"),
        }

        Ok(versions)
    } else {
        Ok(VERSIONS.get().unwrap().to_owned())
    }
}
