use crate::common::utils::file;
use crate::data::models::MinecraftVersionData;

pub async fn download_version_manifest() -> Result<(), Box<dyn std::error::Error>> {
    file::download_as_vec(
        "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json",
        "",
        1,
        "launcher/version-info/version_manifest_v2.json",
        false,
    )
    .await?;
    Ok(())
}

pub async fn get_versions() -> Result<Vec<MinecraftVersionData>, Box<dyn std::error::Error>> {
    let bytes: Vec<u8> =
        file::read_as_vec(format!("launcher/version-info/version_manifest_v2.json").as_str())
            .await?;
    let data: serde_json::Value = serde_json::from_slice(&bytes)?;

    let versions: &Vec<serde_json::Value> =
        data["versions"].as_array().ok_or("Invalid manifest JSON")?;

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
    let bytes: Vec<u8> =
        file::read_as_vec(format!("launcher/version-info/version_manifest_v2.json").as_str())
            .await?;
    let data: serde_json::Value = serde_json::from_slice(&bytes)?;

    let versions: &Vec<serde_json::Value> =
        data["versions"].as_array().ok_or("Invalid manifest JSON")?;

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
