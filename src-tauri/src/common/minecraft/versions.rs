use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use crate::common::utils::directory_checker::check_directory;

#[derive(Serialize, Deserialize)]
pub struct VersionData {
    pub id: String,
    pub url: String,
}

pub async fn download_version_manifest() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path: PathBuf = env::current_exe()?;
    check_directory("launcher/version-info").await;

    let response: serde_json::Value =
        reqwest::get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
            .await?
            .json()
            .await
            .map_err(|e| format!("Failed to get json: {}", e))?;

    let mut version_data: HashMap<&str, Vec<VersionData>> =
        HashMap::<&str, Vec<VersionData>>::new();

    for version in response["versions"].as_array().unwrap() {
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
                    let versions: &mut Vec<VersionData> = version_data.entry(r#type).or_default();
                    versions.push(data);
                }
                _ => continue,
            }
        }
    }

    for (version_type, data) in version_data.iter() {
        write_data_to_file(version_type, data, &exe_path)?;
    }

    Ok(())
}

fn write_data_to_file<T: Serialize>(
    file_name: &str,
    data: &T,
    exe_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path: PathBuf = exe_path
        .parent()
        .ok_or("Failed to get parent directory of current exe path")?
        .join(format!("launcher/version-info/{}.json", file_name));

    let file: File = File::create(&file_path)?;

    let writer: BufWriter<File> = BufWriter::new(file);
    let mut serializer: serde_json::Serializer<BufWriter<File>> =
        serde_json::Serializer::new(writer);

    data.serialize(&mut serializer)?;
    Ok(())
}

pub fn get_versions(r#type: &str) -> Result<Vec<VersionData>, Box<dyn std::error::Error>> {
    let exe_path: PathBuf = env::current_exe()?;

    let file_path: PathBuf = exe_path
        .parent()
        .ok_or("Failed to get parent directory of current exe path")?
        .join(format!("launcher/version-info/{}.json", r#type));

    let file: File = File::open(&file_path)?;

    let reader: BufReader<File> = BufReader::new(file);
    let items: Vec<VersionData> = serde_json::from_reader(reader)?;

    Ok(items)
}
