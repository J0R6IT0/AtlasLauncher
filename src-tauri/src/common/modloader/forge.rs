use std::collections::HashMap;
use std::error::Error;
use std::io::{Cursor, BufReader, BufRead};
use std::process::{Command, Stdio};

use serde::{Serialize, Deserialize};
use serde_json::Value;
use regex::Regex;
use tauri::{AppHandle, Manager};

use crate::common::java::get_java_path::get_java_path;
use crate::common::minecraft::downloader::download_libraries;
use crate::common::utils::directory::check_directory;
use crate::common::utils::file::{self, download_as_vec, extract_file, read_as_value, write_value, library_name_to_path};
use crate::common::utils::file::{
    download_as_json, merge_zips, read_as_vec, write_vec, ChecksumType,
};
use crate::data::constants::{FORGE_VERSION_MANFIEST, EXTRA_FORGE_VERSION_MANIFEST, NET_MINECRAFTFORGE_VERSION_MANIFEST};
use crate::data::models::{DownloadInstanceEventPayload, BaseEventPayload};

#[derive(Serialize, Deserialize, Debug)]
struct ForgeVersions {
    #[serde(flatten)]
    data: HashMap<String, Vec<String>>,
}

pub async fn download_forge(
    id: &str,
    forge: &str,
    instance_name: &str,
    app: &AppHandle
) -> Result<(), Box<dyn std::error::Error>> {
    let id: String = if id.starts_with("_") {
        id.replace("_", "")
    }
    else {
        id.to_string()
    };
    let forge: String = forge.replace("forge-", "");
    let forge_version_manifest: Value = read_as_value(&format!("launcher/meta/net.minecraftforge/{forge}.json")).await?;
    app.emit_all(
        "download",
        DownloadInstanceEventPayload {
            base: BaseEventPayload {
                message: String::from("Downloading Forge libraries"),
                status: String::from("Loading"),
            },
            total: 0,
            downloaded: 0,
            name: instance_name.to_string(),
        },
    )
    .unwrap();
    if let Some(libraries) = forge_version_manifest["libraries"].as_array() {
        let id_copy: String = id.to_string();
        let libraries_copy: Vec<Value> = libraries.clone();
        let handle_copy = app.clone();
        let instance_name_copy: String = instance_name.to_string();
        tauri::async_runtime::spawn(async move {
            match download_libraries(
                &libraries_copy,
                &id_copy,
                false,
                &handle_copy,
                &instance_name_copy
            )
            .await
            {
                Ok(_) => Ok(()),
                Err(err) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("{err}"),
                    )) as Box<dyn std::error::Error + Send + Sync>);
                }
            }
        }).await?.unwrap();
    }
    app.emit_all(
        "download",
        DownloadInstanceEventPayload {
            base: BaseEventPayload {
                message: String::from("Downloading Forge patches"),
                status: String::from("Loading"),
            },
            total: 0,
            downloaded: 0,
            name: instance_name.to_string(),
        },
    )
    .unwrap();
    if let Some(patches) = forge_version_manifest["patches"].as_array() {
        let mut patched_jar_bytes: Vec<u8> = read_as_vec(&format!("versions\\{id}.jar")).await?;
        for patch in patches {
            let url: &str = patch["downloads"]["artifact"]["url"].as_str().unwrap();
            let sha1: &str = patch["downloads"]["artifact"]["sha1"].as_str().unwrap();
            let patch_bytes: Vec<u8> =
                download_as_vec(url, sha1, &file::ChecksumType::SHA1, "", false, false, None).await?;

            patched_jar_bytes =
                merge_zips(&mut patched_jar_bytes, &patch_bytes, true).await?;

        }
        write_vec(&patched_jar_bytes, &format!("versions\\forge-{forge}.jar"))?;

    }
    if let Some(extras) = forge_version_manifest["extra"].as_array() {
        for extra in extras {
            let url: &str = extra["downloads"]["artifact"]["url"].as_str().unwrap();
            let sha1: &str = extra["downloads"]["artifact"]["sha1"].as_str().unwrap();
            let path: String = extra["downloads"]["artifact"]["path"].as_str().unwrap().replace("${game_directory}", check_directory(&format!("instances/{instance_name}")).await.to_str().unwrap());

            download_as_vec(url, sha1, &ChecksumType::SHA1, &path, false, false, None).await?;
        }
    }

    if !forge_version_manifest["id"].as_str().unwrap().contains("forge") {
        return Ok(());
    }

    let forge_copy: String = forge.clone();
    let forge_install_manifest: Result<Value, Box<dyn Error + Send + Sync>> = tauri::async_runtime::spawn(async move {
        match read_as_value(&format!("launcher/meta/net.minecraftforge/{forge_copy}-install.json")).await {
            Ok(value) => Ok(value),
            Err(err) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{err}"),
                )) as Box<dyn std::error::Error + Send + Sync>);
            }
        }
    }).await?;

    let forge_client_path: std::path::PathBuf = check_directory(&format!("libraries/net/minecraftforge/forge/{forge}")).await.join(format!("forge-{forge}-client.jar"));

    if forge_install_manifest.is_ok() && !forge_client_path.is_file() {
        let forge_install_manifest: Value = forge_install_manifest.unwrap();
        let instance_name_copy: String = instance_name.to_string();
        app.emit_all(
            "download",
            DownloadInstanceEventPayload {
                base: BaseEventPayload {
                    message: String::from("Downloading Forge libraries"),
                    status: String::from("Loading"),
                },
                total: 0,
                downloaded: 0,
                name: instance_name.to_string(),
            },
        )
        .unwrap();
        if let Some(libraries) = forge_install_manifest["libraries"].as_array() {

            let id_copy: String = id.to_string();
            let libraries_copy: Vec<Value> = libraries.clone();
            let handle_copy = app.clone();
            tauri::async_runtime::spawn(async move {
                match download_libraries(
                    &libraries_copy,
                    &id_copy,
                    false,
                    &handle_copy,
                    &instance_name_copy
                )
                .await
                {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("{err}"),
                        )) as Box<dyn std::error::Error + Send + Sync>);
                    }
                }
            }).await?.unwrap();
        }
        app.emit_all(
            "download",
            DownloadInstanceEventPayload {
                base: BaseEventPayload {
                    message: String::from("Patching game files"),
                    status: String::from("Loading"),
                },
                total: 0,
                downloaded: 0,
                name: instance_name.to_string(),
            },
        )
        .unwrap();
        if let Some(processors) = forge_install_manifest["processors"].as_array() {
            for processor in processors {
                let mut must_process: bool = false;
                if let Some(sides) = processor["sides"].as_array() {
                    for side in sides {
                        if side.as_str().unwrap() == "client" {
                            must_process = true;
                        }
                    }
                }
                else {
                    must_process = true;
                }
                if must_process {
                    process_processor(processor, &forge_install_manifest["data"], &id, &forge).await?;
                }
            }
        }
    }

    Ok(())
}

pub async fn download_manifest(forge: &str, app: &AppHandle, instance_name: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let forge: String = forge.replace("forge-", "");

    let manifest_path: std::path::PathBuf = check_directory("launcher/meta/net.minecraftforge").await.join(format!("{forge}.json"));
    if manifest_path.is_file() {
        return read_as_value::<Value>(manifest_path.to_str().unwrap()).await;
    }



    let forge_copy: String = forge.clone();
    let forge_manifest: Result<Value, Box<dyn Error + Send + Sync>> = tauri::async_runtime::spawn(async move {
        match download_as_json(
            &format!("https://github.com/J0R6IT0/AtlasLauncherResources/raw/main/meta/net.minecraftforge/{forge_copy}.json"), 
            "", 
            &ChecksumType::SHA1, 
            "", 
            false, 
            false,
            None
        )
        .await
        {
            Ok(result) => Ok(result),
            Err(err) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{err}"),
                )) as Box<dyn std::error::Error + Send + Sync>);
            }
        }
    }).await?;

    if forge_manifest.is_ok() {
        let forge_manifest: Value = forge_manifest.unwrap();
        write_value(&forge_manifest, &format!("launcher/meta/net.minecraftforge/{forge}.json"))?;
        return Ok(forge_manifest);
    }

    let forge_copy: String = forge.clone();
    let md5: Result<String, Box<dyn Error + Send + Sync>> = tauri::async_runtime::spawn(async move {
        match download_as_json(
            &format!("https://files.minecraftforge.net/net/minecraftforge/forge/{forge_copy}/meta.json"), 
            "", &ChecksumType::SHA1, &format!("launcher/meta/net.minecraftforge/{forge_copy}-hashes.json"), 
            false, 
            false,
            None
        )
        .await 
        {
            Ok(hashes) => Ok(hashes["classifiers"]["installer"]["jar"].as_str().unwrap().to_string()),
            Err(err) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{err}"),
                )) as Box<dyn std::error::Error + Send + Sync>);
            }
        }
        
    }).await?;

    app.emit_all(
        "download",
        DownloadInstanceEventPayload {
            base: BaseEventPayload {
                message: String::from("Downloading Forge installer"),
                status: String::from("Loading"),
            },
            total: 0,
            downloaded: 0,
            name: instance_name.to_string(),
        },
    )
    .unwrap();
    let forge_copy: String = forge.clone();
    let handle_copy: AppHandle = app.clone();
    let instance_name_copy: String = instance_name.to_string();
    let installer_bytes: Result<Vec<u8>, Box<dyn Error + Send + Sync>> = tauri::async_runtime::spawn(async move {
        match download_as_vec(
            &format!("https://maven.minecraftforge.net/net/minecraftforge/forge/{forge_copy}/forge-{forge_copy}-installer.jar"),
            &md5.unwrap(),
            &ChecksumType::MD5,
            "",
            false,
            false,
            Some((&handle_copy, &instance_name_copy))
        ).await
        {
            Ok(bytes) => Ok(bytes),
            Err(err) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{err}"),
                )) as Box<dyn std::error::Error + Send + Sync>);
            }
        }
    }).await?;

    app.emit_all(
        "download",
        DownloadInstanceEventPayload {
            base: BaseEventPayload {
                message: String::from("Extracting Forge installer"),
                status: String::from("Loading"),
            },
            total: 0,
            downloaded: 0,
            name: instance_name.to_string(),
        },
    )
    .unwrap();
    if installer_bytes.is_ok() {
        let installer_bytes: Vec<u8> = installer_bytes.unwrap();
        let installer_bytes_copy: Vec<u8> = installer_bytes.clone();
        let install_profile: Vec<u8> = extract_file(&mut Cursor::new(installer_bytes_copy) , "install_profile.json").await?;
        let install_profile: Value = serde_json::from_slice(&install_profile)?;
        let installer_bytes_copy: Vec<u8> = installer_bytes.clone();
        let version: Vec<u8> = extract_file(&mut Cursor::new(installer_bytes_copy) , "version.json").await?;
        let version: Value = serde_json::from_slice(&version)?;
        write_value(&version, &format!("launcher/meta/net.minecraftforge/{forge}.json"))?;
        write_value(&install_profile, &format!("launcher/meta/net.minecraftforge/{forge}-install.json"))?;
        let client_lzma: Vec<u8> = extract_file(&mut Cursor::new(installer_bytes), "client.lzma").await?;
        write_vec(&client_lzma, &format!("launcher/cache/{forge}-client.lzma"))?;
        return Ok(version);
    }


    return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "The Forge installer was not downloaded.",
    )));

}

async fn process_processor(processor: &Value, mappings: &Value, id: &str, forge: &str) -> Result<(), Box<dyn std::error::Error>> {
    let java_path: String = get_java_path(8).await;
    let mut classpath: String = String::from("");
    let mut main_class = String::from("");
    
    if let Some(classes) = processor["classpath"].as_array() {
        for class in classes {

            let class: &str = &library_name_to_path(class.as_str().unwrap());

            classpath += &class;
            classpath += ";";
        }
    }

   if let Some(jar) = processor["jar"].as_str() {
        let class: &str = &library_name_to_path(jar);
        main_class = extract_main_class(class).await?;
        classpath += &class;
   }

   let mut args: Vec<String> = [].to_vec();

   if let Some(processor_args) = processor["args"].as_array() {
        for processor_arg in processor_args {
            let processor_arg: String = processor_arg.as_str().unwrap()
                .replace("{SIDE}", "client")
                .replace("{MINECRAFT_JAR}", check_directory("versions").await.join(format!("{id}.jar")).to_str().unwrap());
            
            if processor_arg.starts_with("[") && processor_arg.ends_with("]") {
                let processor_arg: String = processor_arg.replace("[", "").replace("]", "");
                let path: String = library_name_to_path(&processor_arg);
                args.push(path);
                continue;
            }
            if processor_arg.starts_with("{") && processor_arg.ends_with("}") {
                let mut processor_arg: String = processor_arg.replace("{", "").replace("}", "");
                if let Some(mapping) = mappings[&processor_arg]["client"].as_str() {
                    if mapping.starts_with("[") && mapping.ends_with("]") {
                        let mapping = mapping.replace("[", "").replace("]", "");
                        processor_arg = library_name_to_path(&mapping);
                    }
                    else if mapping.contains("client.lzma") {
                        processor_arg = check_directory("launcher/cache").await.join(format!("{forge}-client.lzma")).to_str().unwrap().to_string();
                    }
                }
                args.push(processor_arg);
                continue;
            }

            args.push(processor_arg.to_string())
        }
   }

   println!("{classpath}");
   println!("{main_class}");
   println!("{:?}", args);

   let mut process: std::process::Child = Command::new(java_path)
    .arg("-cp")
    .arg(classpath)
    .arg(main_class.trim())
    .args(args)
    // .creation_flags(CREATE_NO_WINDOW)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute java process");

    let output: &mut std::process::ChildStdout = process.stdout.as_mut().unwrap();
    let reader: BufReader<&mut std::process::ChildStdout> = BufReader::new(output);
    let lines: std::io::Lines<BufReader<&mut std::process::ChildStdout>> = reader.lines();


    for line in lines {
        let line: String = line.unwrap();
        println!("{line}");
    }

   let status: i32 = process.wait().unwrap().code().unwrap();
   println!("{status}");

    Ok(())
}

async fn extract_main_class(class: &str) -> Result<String, Box<dyn std::error::Error>> {
    let bytes: Vec<u8> = read_as_vec(class).await?;
    let manifest: Vec<u8> = extract_file(&mut Cursor::new(bytes), "MANIFEST.MF").await?;
    let manifest: String = String::from_utf8(manifest)?;
    let re: Regex = Regex::new(r"Main-Class: (.+)").unwrap();
    if let Some(capture) = re.captures(&manifest) {
        return Ok(capture.get(1).map(|m| m.as_str()).unwrap().to_string());
    }

    return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Main Class not found!",
    )));
}

pub async fn download_versions() -> Result<(), Box<dyn std::error::Error>> {
    let mut forge_manifest: String = file::download_as_string(
        FORGE_VERSION_MANFIEST,
        "",
        &file::ChecksumType::SHA1,
        "",
        false,
        true,
        None,
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
        None,
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
    Ok(())
}