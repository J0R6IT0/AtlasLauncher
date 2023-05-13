use crate::common::utils::file::{self, read_as_value, write_value};
use crate::data::constants::{
    BETTER_JSONS_VERSION_MANIFEST, EXTRA_FORGE_VERSION_MANIFEST, FABRIC_VERSION_MANIFEST,
    FORGE_VERSION_MANFIEST, MINECRAFT_VERSION_MANIFEST, NET_FABRICMC_VERSION_MANIFEST,
    NET_MINECRAFTFORGE_VERSION_MANIFEST, NET_MINECRAFT_VERSION_MANIFEST,
};
use crate::data::models::MinecraftVersionData;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct ForgeVersions {
    #[serde(flatten)]
    data: HashMap<String, Vec<String>>,
}

pub async fn download_version_manifests() -> Result<(), Box<dyn std::error::Error>> {
    // vanilla + betterjsons
    let version_manifest: Value = file::download_as_json(
        MINECRAFT_VERSION_MANIFEST,
        "",
        &file::ChecksumType::SHA1,
        "",
        false,
        true,
    )
    .await?;

    let better_jsons: Value = file::download_as_json(
        BETTER_JSONS_VERSION_MANIFEST,
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

    file::write_value(&versions, NET_MINECRAFT_VERSION_MANIFEST)?;

    // forge

    let mut forge_manifest: String = file::download_as_string(
        FORGE_VERSION_MANFIEST,
        "",
        &file::ChecksumType::SHA1,
        "",
        false,
        true,
    )
    .await?;

    forge_manifest = forge_manifest
        .replace("{", "[{")
        .replace("}", "}]")
        .replace("],", "]},{")
        .replace("1.4.0", "1.4")
        .replace("1.7.10_pre4", "1.7.10-pre4");

    let mut final_forge_manifest: Vec<ForgeVersions> = serde_json::from_str(&forge_manifest)?;

    let extra_forge_versions = file::download_as_json(
        EXTRA_FORGE_VERSION_MANIFEST,
        "",
        &file::ChecksumType::SHA1,
        "",
        false,
        false,
    )
    .await?;

    let mut extra_forge_versions: Vec<Value> = extra_forge_versions.as_array().unwrap().to_owned();
    extra_forge_versions.reverse();

    'extra: for extra_forge_version in extra_forge_versions {
        let mc_id: &str = extra_forge_version["mc_id"].as_str().unwrap();
        let versions: Vec<String> = extra_forge_version["versions"]
            .as_array()
            .unwrap()
            .into_iter()
            .map(|version| version["id"].as_str().unwrap().to_string())
            .collect();

        for final_forge_version in final_forge_manifest.iter_mut() {
            let key: String = final_forge_version
                .data
                .keys()
                .next()
                .unwrap()
                .replace("\"", "");
            let og_versions: &mut Vec<String> =
                final_forge_version.data.values_mut().next().unwrap();
            if mc_id == key {
                *og_versions = versions.clone();
                continue 'extra;
            }
        }
        let mut data: HashMap<String, Vec<String>> = HashMap::new();
        data.insert(String::from(mc_id), versions);

        final_forge_manifest.insert(0, ForgeVersions { data });
    }

    write_value(&final_forge_manifest, NET_MINECRAFTFORGE_VERSION_MANIFEST)?;

    // fabric

    file::download_as_json(
        FABRIC_VERSION_MANIFEST,
        "",
        &file::ChecksumType::SHA1,
        NET_FABRICMC_VERSION_MANIFEST,
        false,
        true,
    )
    .await?;

    Ok(())
}

pub async fn get_versions() -> Result<Vec<MinecraftVersionData>, Box<dyn std::error::Error>> {
    let data: serde_json::Value = file::read_as_value(NET_MINECRAFT_VERSION_MANIFEST)
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
    let data: serde_json::Value = file::read_as_value(NET_MINECRAFT_VERSION_MANIFEST)
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

pub async fn get_forge_versions() -> Result<Value, Box<dyn std::error::Error>> {
    let data: serde_json::Value = file::read_as_value(NET_MINECRAFTFORGE_VERSION_MANIFEST)
        .await
        .unwrap();

    Ok(data)
}

pub async fn get_fabric_mc_versions() -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let manifest: Value = read_as_value(NET_FABRICMC_VERSION_MANIFEST).await?;
    if manifest["game"].is_array() {
        return Ok(manifest["game"].as_array().unwrap().to_owned());
    } else {
        return Ok([].to_vec());
    }
}

pub async fn get_fabric_loader_versions() -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let manifest: Value = read_as_value(NET_FABRICMC_VERSION_MANIFEST).await?;
    if manifest["loader"].is_array() {
        return Ok(manifest["loader"].as_array().unwrap().to_owned());
    } else {
        return Ok([].to_vec());
    }
}
