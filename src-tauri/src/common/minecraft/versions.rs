use crate::common::utils::file;
use crate::data::models::{ForgeVersionData, ForgeVersionsData, MinecraftVersionData};
use serde_json::Value;

pub async fn download_version_manifests() -> Result<(), Box<dyn std::error::Error>> {
    // vanilla + betterjsons
    let version_manifest: Value = file::download_as_json(
        "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json",
        "",
        &file::ChecksumType::SHA1,
        "",
        false,
        true,
    )
    .await?;

    let better_jsons: Value = file::download_as_json(
        "https://raw.githubusercontent.com/MCPHackers/BetterJSONs/main/version_manifest_v2.json",
        "",
        &file::ChecksumType::SHA1,
        "",
        false,
        true,
    )
    .await?;

    let mut versions: Vec<Value> = version_manifest["versions"].as_array().unwrap().to_owned();

    // the last 349 entries are already in betterjsons
    versions.truncate(versions.len() - 349);
    let better_jsons_versions: Vec<Value> = better_jsons["versions"].as_array().unwrap().to_owned();

    versions.extend(better_jsons_versions);
    versions.sort_by_key(|value| value["releaseTime"].as_str().unwrap_or("").to_lowercase());
    versions.reverse();

    // cherrypick some versions to use with forge, as they are not compatible with the custom launch wrapper
    versions.append(&mut vec![
        serde_json::from_str("{\"id\": \"_1.3.2\",\"sha1\": \"598eedd6f67db4aefbae6ed119029e3d7373ecf5\",\"url\": \"https://piston-meta.mojang.com/v1/packages/598eedd6f67db4aefbae6ed119029e3d7373ecf5/1.3.2.json\"}").unwrap(),
        serde_json::from_str("{\"id\": \"_1.4\",\"sha1\": \"d979a4671611bf8704c0a2a0cf09964ca25eefd7\",\"url\": \"https://piston-meta.mojang.com/v1/packages/d979a4671611bf8704c0a2a0cf09964ca25eefd7/1.4.json\"}").unwrap(),
        serde_json::from_str("{\"id\": \"_1.4.1\",\"sha1\": \"14c3ba517b5baabdfc61b60eb49d9aa7da012906\",\"url\": \"https://piston-meta.mojang.com/v1/packages/14c3ba517b5baabdfc61b60eb49d9aa7da012906/1.4.1.json\"}").unwrap(),
        serde_json::from_str("{\"id\": \"_1.4.2\",\"sha1\": \"2fd77aa19aba2860bbf4c1fd9f84f232703dd287\",\"url\": \"https://piston-meta.mojang.com/v1/packages/2fd77aa19aba2860bbf4c1fd9f84f232703dd287/1.4.2.json\"}").unwrap(),
        serde_json::from_str("{\"id\": \"_1.4.3\",\"sha1\": \"3ab416ac64dac1a6123402a8aabd8ef3caeef087\",\"url\": \"https://piston-meta.mojang.com/v1/packages/3ab416ac64dac1a6123402a8aabd8ef3caeef087/1.4.3.json\"}").unwrap(),
        serde_json::from_str("{\"id\": \"_1.4.4\",\"sha1\": \"f7de827181036b09444abb6b64c1fcc663b8e98e\",\"url\": \"https://piston-meta.mojang.com/v1/packages/f7de827181036b09444abb6b64c1fcc663b8e98e/1.4.4.json\"}").unwrap(),
        serde_json::from_str("{\"id\": \"_1.4.5\",\"sha1\": \"d64a902a48a6a618f9a0a82c183be454e7a1f23b\",\"url\": \"https://piston-meta.mojang.com/v1/packages/d64a902a48a6a618f9a0a82c183be454e7a1f23b/1.4.5.json\"}").unwrap(),
        serde_json::from_str("{\"id\": \"_1.4.6\",\"sha1\": \"09832797138da79745ade734da775f44c254066b\",\"url\": \"https://piston-meta.mojang.com/v1/packages/09832797138da79745ade734da775f44c254066b/1.4.6.json\"}").unwrap(),
        serde_json::from_str("{\"id\": \"_1.4.7\",\"sha1\": \"7aa8e9aeacf4e1076bfd81c096f78de9b883ebe6\",\"url\": \"https://piston-meta.mojang.com/v1/packages/7aa8e9aeacf4e1076bfd81c096f78de9b883ebe6/1.4.7.json\"}").unwrap(),
        serde_json::from_str("{\"id\": \"_1.5\",\"sha1\": \"bb882e3d97bee9c5b5e486da04b85f977e770150\",\"url\": \"https://piston-meta.mojang.com/v1/packages/bb882e3d97bee9c5b5e486da04b85f977e770150/1.5.json\"}").unwrap(),
        serde_json::from_str("{\"id\": \"_1.5.1\",\"sha1\": \"3c514114d9c2a3ea78f72c4f9fb4eeb56747135a\",\"url\": \"https://piston-meta.mojang.com/v1/packages/3c514114d9c2a3ea78f72c4f9fb4eeb56747135a/1.5.1.json\"}").unwrap(),
        serde_json::from_str("{\"id\": \"_1.5.2\",\"sha1\": \"924a2dcd8bdc31f8e9d36229811c298b3537bbc7\",\"url\": \"https://piston-meta.mojang.com/v1/packages/924a2dcd8bdc31f8e9d36229811c298b3537bbc7/1.5.2.json\"}").unwrap()

    ]);

    file::write_value(
        &versions,
        "launcher/meta/net.minecraft/version_manifest.json",
    )?;

    // forge

    file::download_as_json(
        "https://github.com/J0R6IT0/AtlasLauncherResources/raw/main/meta/net.minecraftforge/version_manifest.json",
        "",
        &file::ChecksumType::SHA1,
        "launcher/meta/net.minecraftforge/version_manifest.json",
        false,
        true,
    )
    .await
    .unwrap();

    Ok(())
}

pub async fn get_versions() -> Result<Vec<MinecraftVersionData>, Box<dyn std::error::Error>> {
    let data: serde_json::Value =
        file::read_as_value(format!("launcher/meta/net.minecraft/version_manifest.json").as_str())
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
    let data: serde_json::Value =
        file::read_as_value(format!("launcher/meta/net.minecraft/version_manifest.json").as_str())
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

pub async fn get_forge_versions() -> Result<Vec<ForgeVersionsData>, Box<dyn std::error::Error>> {
    let data: serde_json::Value = file::read_as_value(
        format!("launcher/meta/net.minecraftforge/version_manifest.json").as_str(),
    )
    .await
    .unwrap();

    let versions: &Vec<serde_json::Value> = data.as_array().ok_or("Invalid manifest JSON")?;

    let forge_versions: Vec<ForgeVersionsData> = versions
        .iter()
        .map(|version| ForgeVersionsData {
            mc_id: version["mc_id"].as_str().unwrap_or("").to_owned(),
            versions: version["versions"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|v| serde_json::from_value::<ForgeVersionData>(v.to_owned()).ok())
                .collect(),
        })
        .collect();

    Ok(forge_versions)
}

pub async fn get_forge_version(
    id: &str,
    forge: &str,
) -> Result<ForgeVersionData, Box<dyn std::error::Error>> {
    let data: serde_json::Value = file::read_as_value(
        format!("launcher/meta/net.minecraftforge/version_manifest.json").as_str(),
    )
    .await
    .unwrap();

    let versions: &Vec<serde_json::Value> = data.as_array().ok_or("Invalid manifest JSON")?;

    let forge_version: Option<ForgeVersionData> = versions.iter().find_map(|version| {
        let version_id: &str = version["mc_id"].as_str()?;
        if version_id == id {
            let releases: &Vec<Value> = version["versions"]
                .as_array()
                .ok_or("Invalid manifest JSON")
                .unwrap();
            releases.iter().find_map(|release| {
                let release_id: &str = release["id"].as_str()?;
                if release_id == forge {
                    Some(ForgeVersionData {
                        id: release_id.to_owned(),
                        url: release["url"].as_str().unwrap_or("").to_owned(),
                        sha1: release["sha1"].as_str().unwrap_or("").to_owned(),
                        size: release["size"].as_str().unwrap_or("").to_owned(),
                    })
                } else {
                    None
                }
            })
        } else {
            None
        }
    });

    Ok(forge_version.unwrap())
}
