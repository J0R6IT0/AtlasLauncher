use crate::common::utils::file;
use crate::data::models::MinecraftVersionData;
use serde_json::Value;

pub async fn download_version_manifest() -> Result<(), Box<dyn std::error::Error>> {
    let version_manifest: Value = file::download_as_json(
        "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json",
        "",
        &file::ChecksumType::Sha1,
        "launcher/version-info/version_manifest_v2.json",
        false,
        true,
    )
    .await?;

    let hidden_version_manifest: Value = file::download_as_json(
        "https://github.com/J0R6IT0/AtlasLauncherResources/releases/download/Manifest/hidden_versions.json",
        "",
        &file::ChecksumType::Sha1,
        "launcher/version-info/hidden_version_manifest.json",
        false,
        true,
    )
    .await?;

    let mut versions: Vec<Value> = version_manifest["versions"].as_array().unwrap().to_owned();
    let hidden_versions: Vec<Value> = hidden_version_manifest["versions"]
        .as_array()
        .unwrap()
        .to_owned();

    versions.extend(hidden_versions);
    versions.sort_by_key(|value| value["releaseTime"].as_str().unwrap_or("").to_lowercase());
    versions.reverse();

    file::write_value(
        &versions,
        "launcher/version-info/complete_version_manifest.json",
    )?;

    Ok(())
}

pub async fn get_versions() -> Result<Vec<MinecraftVersionData>, Box<dyn std::error::Error>> {
    let data: serde_json::Value = file::read_as_value(
        format!("launcher/version-info/complete_version_manifest.json").as_str(),
    )
    .await
    .unwrap();

    let versions: &Vec<serde_json::Value> = data.as_array().ok_or("Invalid manifest JSON")?;

    let minecraft_versions: Vec<MinecraftVersionData> = versions
        .iter()
        .map(|version| MinecraftVersionData {
            id: version["id"].as_str().unwrap_or("").to_owned(),
            url: version["url"].as_str().unwrap_or("").to_owned(),
            r#type: version["type"].as_str().unwrap_or("").to_owned(),
        })
        .collect();

    Ok(minecraft_versions)
}

pub async fn get_version(id: &str) -> Result<MinecraftVersionData, Box<dyn std::error::Error>> {
    let data: serde_json::Value = file::read_as_value(
        format!("launcher/version-info/complete_version_manifest.json").as_str(),
    )
    .await
    .unwrap();

    let versions: &Vec<serde_json::Value> = data.as_array().ok_or("Invalid manifest JSON")?;

    let minecraft_version: Option<MinecraftVersionData> = versions.iter().find_map(|version| {
        let version_id: &str = version["id"].as_str()?;
        if version_id == id {
            Some(MinecraftVersionData {
                id: version_id.to_owned(),
                url: version["url"].as_str().unwrap_or("").to_owned(),
                r#type: version["type"].as_str().unwrap_or("").to_owned(),
            })
        } else {
            None
        }
    });

    Ok(minecraft_version.unwrap())
}
