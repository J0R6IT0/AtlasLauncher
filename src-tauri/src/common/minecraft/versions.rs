use std::error::Error;

use crate::common::modloader::forge;
use crate::common::utils::file::{self, read_as_value};
use crate::data::constants::{
    BETTER_JSONS_VERSION_MANIFEST, EXTRA_VERSION_MANIFEST, FABRIC_VERSION_MANIFEST,
    MINECRAFT_VERSION_MANIFEST, NET_FABRICMC_VERSION_MANIFEST, NET_MINECRAFTFORGE_VERSION_MANIFEST,
    NET_MINECRAFT_VERSION_MANIFEST, ORG_QUILTMC_VERSION_MANIFEST, QUILT_VERSION_MANIFEST,
};
use crate::data::models::MinecraftVersionData;
use futures::future::join_all;
use serde_json::Value;
use tauri::async_runtime;

pub async fn download_version_manifests() -> Result<(), Box<dyn std::error::Error>> {
    let mut download_tasks: Vec<
        async_runtime::JoinHandle<Result<(), Box<dyn Error + Send + Sync>>>,
    > = vec![];

    // vanilla + betterjsons + extra
    let download_task: async_runtime::JoinHandle<
        Result<(), Box<dyn std::error::Error + Send + Sync>>,
    > = tauri::async_runtime::spawn(async move {
        let version_manifest: Value = match file::download_as_json(
            MINECRAFT_VERSION_MANIFEST,
            "",
            &file::ChecksumType::SHA1,
            "",
            false,
            true,
            None,
        )
        .await
        {
            Ok(value) => value,
            Err(err) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{err}"),
                ))
                    as Box<dyn std::error::Error + Send + Sync>);
            }
        };

        let better_jsons: Value = match file::download_as_json(
            BETTER_JSONS_VERSION_MANIFEST,
            "",
            &file::ChecksumType::SHA1,
            "",
            false,
            true,
            None,
        )
        .await
        {
            Ok(value) => value,
            Err(err) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{err}"),
                ))
                    as Box<dyn std::error::Error + Send + Sync>);
            }
        };

        let extra_version_manifest: Value = match file::download_as_json(
            EXTRA_VERSION_MANIFEST,
            "",
            &file::ChecksumType::SHA1,
            "",
            false,
            true,
            None,
        )
        .await
        {
            Ok(value) => value,
            Err(err) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{err}"),
                ))
                    as Box<dyn std::error::Error + Send + Sync>);
            }
        };
        let mut versions: Vec<Value> = version_manifest["versions"].as_array().unwrap().to_owned();

        // cherrypick some versions to use with forge, as they are not compatible with the custom launch wrapper
        let mut duplicate_versions: Vec<Value> = vec![];
        let versions_to_keep: Vec<&str> = vec![
            "1.3.2", "1.4", "1.4.1", "1.4.2", "1.4.3", "1.4.4", "1.4.5", "1.4.6", "1.4.7", "1.5",
            "1.5.1", "1.5.2",
        ];

        for version in versions.iter() {
            if let Some(id) = version["id"].as_str() {
                if versions_to_keep.contains(&id) {
                    let sha1: &str = version["sha1"].as_str().unwrap();
                    let url: &str = version["url"].as_str().unwrap();
                    duplicate_versions.push(
                        serde_json::from_str(&format!(
                            "{{\"id\": \"_{id}\",\"sha1\": \"{sha1}\",\"url\": \"{url}\"}}"
                        ))
                        .unwrap(),
                    )
                }
            }
        }

        // the last 349 entries are already in betterjsons
        versions.truncate(versions.len() - 349);
        let better_jsons_versions: Vec<Value> =
            better_jsons["versions"].as_array().unwrap().to_owned();
        let extra_versions: Vec<Value> = extra_version_manifest["versions"]
            .as_array()
            .unwrap()
            .to_owned();

        versions.extend(better_jsons_versions);
        versions.extend(extra_versions);
        versions.sort_by_key(|value| value["releaseTime"].as_str().unwrap_or("").to_lowercase());
        versions.reverse();
        versions.extend(duplicate_versions);

        match file::write_value(&versions, NET_MINECRAFT_VERSION_MANIFEST) {
            Ok(_) => Ok(()),
            Err(err) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{err}"),
                ))
                    as Box<dyn std::error::Error + Send + Sync>);
            }
        }
    });

    download_tasks.push(download_task);

    // forge
    let download_task: async_runtime::JoinHandle<
        Result<(), Box<dyn std::error::Error + Send + Sync>>,
    > = tauri::async_runtime::spawn(async move {
        match forge::download_versions().await {
            Ok(_) => Ok(()),
            Err(err) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{err}"),
                ))
                    as Box<dyn std::error::Error + Send + Sync>);
            }
        }
    });

    download_tasks.push(download_task);

    // fabric
    let download_task: async_runtime::JoinHandle<
        Result<(), Box<dyn std::error::Error + Send + Sync>>,
    > = tauri::async_runtime::spawn(async move {
        match file::download_as_json(
            FABRIC_VERSION_MANIFEST,
            "",
            &file::ChecksumType::SHA1,
            NET_FABRICMC_VERSION_MANIFEST,
            false,
            true,
            None,
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
    });

    download_tasks.push(download_task);

    // quilt
    let download_task: async_runtime::JoinHandle<
        Result<(), Box<dyn std::error::Error + Send + Sync>>,
    > = tauri::async_runtime::spawn(async move {
        match file::download_as_json(
            QUILT_VERSION_MANIFEST,
            "",
            &file::ChecksumType::SHA1,
            ORG_QUILTMC_VERSION_MANIFEST,
            false,
            true,
            None,
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
    });

    download_tasks.push(download_task);
    join_all(download_tasks).await;

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

pub async fn get_quilt_mc_versions() -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let manifest: Value = read_as_value(ORG_QUILTMC_VERSION_MANIFEST).await?;
    if manifest["game"].is_array() {
        return Ok(manifest["game"].as_array().unwrap().to_owned());
    } else {
        return Ok([].to_vec());
    }
}

pub async fn get_quilt_loader_versions() -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let manifest: Value = read_as_value(ORG_QUILTMC_VERSION_MANIFEST).await?;
    if manifest["loader"].is_array() {
        return Ok(manifest["loader"].as_array().unwrap().to_owned());
    } else {
        return Ok([].to_vec());
    }
}
