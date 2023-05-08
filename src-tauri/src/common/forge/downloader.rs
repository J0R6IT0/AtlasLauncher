use std::io::Cursor;

use serde_json::Value;

use crate::common::minecraft::versions::get_forge_version;
use crate::common::utils::directory::check_directory;
use crate::common::utils::file::{self, download_as_vec, extract_json, read_as_value};
use crate::common::utils::file::{
    download_as_json, merge_zips, read_as_vec, write_vec, ChecksumType,
};

pub async fn download_forge(
    id: &str,
    forge: &str,
    instance_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let id: &str = if id.starts_with("_") { &id[1..] } else { id };
    let forge_data: crate::data::models::ForgeVersionData = get_forge_version(id, forge).await?;
    let forge_manifest: Value = download_as_json(
        &forge_data.url,
        "",
        &ChecksumType::SHA1,
        &format!("launcher/meta/net.minecraftforge/{}.json", forge_data.id),
        false,
        false,
    )
    .await?;

    if forge_manifest["install"].is_object() {
        
    } else {
        download_pre_one_six(forge_manifest, id, forge, instance_name).await?;
    }

    Ok(())
}

async fn download_pre_one_six(
    forge_manifest: Value,
    id: &str,
    forge: &str,
    instance_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut patched_jar_bytes: Vec<u8> = read_as_vec(&format!("versions\\{id}.jar")).await?;

    if let Some(patches) = forge_manifest["patches"].as_array() {
        for patch in patches {
            let url: &str = patch["downloads"]["artifact"]["url"].as_str().unwrap();
            let sha1: &str = patch["downloads"]["artifact"]["sha1"].as_str().unwrap();
            let patch_bytes: Vec<u8> =
                download_as_vec(url, sha1, &file::ChecksumType::SHA1, "", false, false).await?;

            patched_jar_bytes = merge_zips(&mut patched_jar_bytes, &patch_bytes).await?;
        }
    };

    let forge_url: &str = forge_manifest["downloads"]["client"]["url"]
        .as_str()
        .unwrap();
    let forge_sha1: &str = forge_manifest["downloads"]["client"]["sha1"]
        .as_str()
        .unwrap();

    let forge_bytes: Vec<u8> = download_as_vec(
        forge_url,
        forge_sha1,
        &file::ChecksumType::SHA1,
        "",
        false,
        false,
    )
    .await?;

    patched_jar_bytes = merge_zips(&mut patched_jar_bytes, &forge_bytes).await?;

    write_vec(&patched_jar_bytes, &format!("versions\\forge-{forge}.jar"))?;

    if let Some(libraries) = forge_manifest["libraries"].as_array() {
        for library in libraries {
            let url: &str = library["downloads"]["artifact"]["url"].as_str().unwrap();
            let sha1: &str = library["downloads"]["artifact"]["sha1"].as_str().unwrap();
            let path: &str = &library["downloads"]["artifact"]["path"]
                .as_str()
                .unwrap()
                .replace("${game_directory}", &format!("instances\\{instance_name}"));
            download_as_vec(url, sha1, &file::ChecksumType::SHA1, path, false, false).await?;
        }
    }
    Ok(())
}

pub async fn download_manifest(id: &str, forge: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let forge: String = forge.replace("forge-", "");
    let forge_data: crate::data::models::ForgeVersionData =
        get_forge_version(&id, &forge).await.unwrap();

    let exists: std::path::PathBuf = check_directory("launcher/meta/net.minecraftforge")
        .await
        .join(format!("{forge}.json"));

    if exists.exists() {
        return read_as_value(&format!("launcher/meta/net.minecraftforge/{forge}.json")).await;
    }

    if forge_data.url.starts_with(
        "https://github.com/J0R6IT0/AtlasLauncherResources/raw/main/meta/net.minecraftforge/",
    ) {
        let forge_manifest: Value = download_as_json(
            &forge_data.url,
            "",
            &ChecksumType::SHA1,
            &format!("launcher/meta/net.minecraftforge/{}.json", forge_data.id),
            false,
            false,
        )
        .await
        .unwrap();
        return Ok(forge_manifest);
    } else {
        let installer_bytes: Vec<u8> = download_as_vec(
            &forge_data.url,
            &forge_data.sha1,
            &ChecksumType::SHA1,
            "",
            false,
            false,
        )
        .await?;
        let json: Value = extract_json(&mut Cursor::new(installer_bytes), "install_profile.json")
            .await?
            .unwrap();

        write_vec(
            &serde_json::to_vec(&json).unwrap(),
            &format!("launcher/meta/net.minecraftforge/{}.json", forge_data.id),
        )?;
        return Ok(json);
    }
}
