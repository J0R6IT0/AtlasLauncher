use std::error::Error;
use std::io::{Cursor, BufReader, BufRead};
use std::process::{Command, Stdio};

use serde_json::Value;
use regex::Regex;

use crate::common::java::get_java_path::get_java_path;
use crate::common::minecraft::downloader::download_libraries;
use crate::common::utils::directory::check_directory;
use crate::common::utils::file::{self, download_as_vec, extract_file, read_as_value, write_value, library_name_to_path};
use crate::common::utils::file::{
    download_as_json, merge_zips, read_as_vec, write_vec, ChecksumType,
};

pub async fn download_forge(
    id: &str,
    forge: &str,
    instance_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let id: String = if id.starts_with("_") {
        id.replace("_", "")
    }
    else {
        id.to_string()
    };
    let forge: String = forge.replace("forge-", "");
    let forge_version_manifest: Value = read_as_value(&format!("launcher/meta/net.minecraftforge/{forge}.json")).await?;
    if let Some(libraries) = forge_version_manifest["libraries"].as_array() {
        let id_copy: String = id.to_string();
        let libraries_copy: Vec<Value> = libraries.clone();
        tauri::async_runtime::spawn(async move {
            match download_libraries(
                &libraries_copy,
                &id_copy,
                false
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
    if let Some(patches) = forge_version_manifest["patches"].as_array() {
        let mut patched_jar_bytes: Vec<u8> = read_as_vec(&format!("versions\\{id}.jar")).await?;
        for patch in patches {
            let url: &str = patch["downloads"]["artifact"]["url"].as_str().unwrap();
            let sha1: &str = patch["downloads"]["artifact"]["sha1"].as_str().unwrap();
            let patch_bytes: Vec<u8> =
                download_as_vec(url, sha1, &file::ChecksumType::SHA1, "", false, false).await?;

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

            download_as_vec(url, sha1, &ChecksumType::SHA1, &path, false, false).await?;
        }
    }

    if !forge_version_manifest["id"].as_str().unwrap().contains("forge") {
        return Ok(());
    }

    let forge_copy = forge.clone();
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

    if forge_install_manifest.is_ok() {
        let forge_install_manifest: Value = forge_install_manifest.unwrap();
        if let Some(libraries) = forge_install_manifest["libraries"].as_array() {

            let id_copy: String = id.to_string();
            let libraries_copy: Vec<Value> = libraries.clone();
            tauri::async_runtime::spawn(async move {
                match download_libraries(
                    &libraries_copy,
                    &id_copy,
                    false
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

pub async fn download_manifest(forge: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let forge: String = forge.replace("forge-", "");

    let exists: std::path::PathBuf = check_directory("launcher/meta/net.minecraftforge")
        .await
        .join(format!("{forge}.json"));

    if exists.exists() {
        return read_as_value(&format!("launcher/meta/net.minecraftforge/{forge}.json")).await;
    }

    let forge_copy: String = forge.clone();
    let forge_manifest: Result<Value, Box<dyn Error + Send + Sync>> = tauri::async_runtime::spawn(async move {
        match download_as_json(
            &format!("https://github.com/J0R6IT0/AtlasLauncherResources/raw/main/meta/net.minecraftforge/{forge_copy}.json"), 
            "", 
            &ChecksumType::SHA1, 
            "", 
            false, 
            false
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
        let forge_manifest = forge_manifest.unwrap();
        write_value(&forge_manifest, &format!("launcher/meta/net.minecraftforge/{forge}.json"))?;
        return Ok(forge_manifest);
    }

    let forge_copy: String = forge.clone();
    let md5: Result<String, Box<dyn Error + Send + Sync>> = tauri::async_runtime::spawn(async move {
        match download_as_json(
            &format!("https://files.minecraftforge.net/net/minecraftforge/forge/{forge_copy}/meta.json"), 
            "", &ChecksumType::SHA1, &format!("launcher/meta/net.minecraftforge/{forge_copy}-hashes.json"), 
            false, 
            false
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

    let forge_copy: String = forge.clone();
    let installer_bytes: Result<Vec<u8>, Box<dyn Error + Send + Sync>> = tauri::async_runtime::spawn(async move {
        match download_as_vec(
            &format!("https://maven.minecraftforge.net/net/minecraftforge/forge/{forge_copy}/forge-{forge_copy}-installer.jar"),
            &md5.unwrap(),
            &ChecksumType::MD5,
            "",
            false,
            false
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