use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::common::utils::file;

#[derive(Serialize, Deserialize)]
pub struct VersionData {
    pub id: String,
    pub url: String,
}

pub async fn download_version_manifest() -> Result<(), Box<dyn std::error::Error>> {
    let json: Value = file::download_as_json(
        "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json",
        "",
        1,
        "",
        false,
    )
    .await?;

    let mut version_data: HashMap<&str, Vec<VersionData>> =
        HashMap::<&str, Vec<VersionData>>::new();

    for version in json["versions"].as_array().unwrap() {
        if let (Some(id), Some(r#type), Some(url)) = (
            version["id"].as_str(),
            version["type"].as_str(),
            version["url"].as_str(),
        ) {
            let data: VersionData = VersionData {
                id: id.to_owned(),
                url: url.to_owned(),
            };
            match r#type {
                "release" | "snapshot" | "old_beta" | "old_alpha" => {
                    version_data.entry(r#type).or_default().push(data);
                }
                _ => continue,
            }
        }
    }

    for (version_type, data) in version_data.iter() {
        file::write_vec(
            &serde_json::to_vec(&data).unwrap(),
            format!("launcher/version-info/{}.json", version_type).as_str(),
        )?;
    }

    Ok(())
}

pub async fn get_versions(r#type: &str) -> Result<Vec<VersionData>, Box<dyn std::error::Error>> {
    let bytes: Vec<u8> =
        file::read_as_vec(format!("launcher/version-info/{}.json", r#type).as_str()).await?;
    let content: Vec<VersionData> = serde_json::from_slice(&bytes)?;

    Ok(content)
}
