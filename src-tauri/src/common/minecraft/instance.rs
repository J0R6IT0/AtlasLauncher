use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    env,
    fs::{self, DirEntry},
    path::PathBuf,
    process::Command,
    os::windows::process::CommandExt,
};


use crate::utils::{directory_checker::check_directory, json_to_file};
use crate::{common::auth::login::get_active_account_info, minecraft::downloader};
use crate::{
    common::{utils::file_to_json},
    java::{downloader as javaDownloader, get_java_path::get_java_path},
};

use tauri::Manager;

#[derive(Clone, Serialize)]
pub struct CreateInstanceEventPayload {
    pub name: String,
    pub version: String,
    pub message: String,
    pub status: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InstanceInfo {
    pub name: String,
    pub version: String,
}

pub async fn create_instance(
    version_type: &str,
    version: &str,
    name: &str,
    app: &tauri::AppHandle,
) {
    check_directory(format!("instances/{name}").as_str()).await;

    let instance_info: String = format!(
        r#"
            {{
                "name": "{name}",
                "version": "{version}"
            }}
        "#
    );

    json_to_file::save(
        &instance_info,
        &format!("instances/{name}/atlas_instance.json"),
    );

    app.emit_all(
        "create_instance",
        CreateInstanceEventPayload {
            version: String::from(version),
            name: String::from(name),
            message: format!("Downloading Java"),
            status: String::from("Loading"),
        },
    )
    .unwrap();

    javaDownloader::download(8).await.unwrap();
    javaDownloader::download(17).await.unwrap();

    app.emit_all(
        "create_instance",
        CreateInstanceEventPayload {
            version: String::from(version),
            name: String::from(name),
            message: format!("Downloading game files"),
            status: String::from("Loading"),
        },
    )
    .unwrap();

    downloader::download(version_type, version, name).await.unwrap();

    app.emit_all(
        "create_instance",
        CreateInstanceEventPayload {
            version: String::from(version),
            name: String::from(name),
            message: format!("Instance created successfully"),
            status: String::from("Success"),
        },
    )
    .unwrap();
}

pub async fn launch_instance(name: &str) {
    let instance_info: Value =
        file_to_json::read(format!("instances/{name}/atlas_instance.json").as_str()).unwrap();
    let version: &str = instance_info["version"].as_str().unwrap();

    let version_info: Value =
        file_to_json::read(format!("versions/{version}/{version}.json").as_str()).unwrap();
    let java_version: u64 = match version_info["javaVersion"]["majorVersion"].as_u64() {
        Some(java_version) => java_version,
        None => 8,
    };


    let java_path = get_java_path(java_version.try_into().unwrap()).await;

    let version_path = String::from(
        check_directory(format!("versions/{version}").as_str())
            .await
            .join(format!("{version}.jar"))
            .to_str()
            .unwrap(),
    );
    let libraries = get_libraries(&version_info["libraries"]).await;

    let cp = format!("{version_path};{libraries}");

    let active_user = get_active_account_info().await;

    let asset_index = version_info["assetIndex"]["id"].as_str().unwrap();
    let main_class = version_info["mainClass"].as_str().unwrap();

    let arguments = match version_info["minecraftArguments"].as_str() {
        Some(arguments) => arguments,
        None => "--username ${auth_player_name} --version ${version_name} --gameDir ${game_directory} --assetsDir ${assets_root} --assetIndex ${assets_index_name} --uuid ${auth_uuid} --accessToken ${auth_access_token} --userType ${user_type}"
    };
    let replaced_arguments = arguments
        .replace("${auth_player_name}", active_user["username"].as_str().unwrap())
        .replace("${auth_session}", active_user["access_token"].as_str().unwrap())
        .replace("${game_directory}", check_directory(format!("instances/{name}").as_str()).await.to_str().unwrap())
        .replace("${game_assets}", check_directory(format!("{}", if asset_index == "legacy" || asset_index == "pre-1.6" { "assets/virtual/legacy" } else { "assets" }).as_str()).await.to_str().unwrap())
        .replace("${version_name}", version)
        .replace("${assets_root}", check_directory(format!("{}", if asset_index == "legacy" || asset_index == "pre-1.6" { "assets/virtual/legacy" } else { "assets" }).as_str()).await.to_str().unwrap())
        .replace("${assets_index_name}", asset_index)
        .replace("${auth_uuid}", active_user["uuid"].as_str().unwrap())
        .replace("${auth_access_token}", active_user["access_token"].as_str().unwrap())
        .replace("${user_properties}", "{}")
        .replace("${user_type}", "msa");


    let args: Vec<&str> = replaced_arguments.split_whitespace().collect();

    // const CREATE_NO_WINDOW: u32 = 0x08000000;

    let mut process = Command::new(java_path)
        .arg("-cp")
        .arg(cp)
        .args([
            "-Xmx2G",
            "-Xms2G",
            "-XX:+UnlockExperimentalVMOptions",
            "-XX:+UseG1GC",
            "-XX:G1NewSizePercent=20",
            "-XX:G1ReservePercent=20",
            "-XX:MaxGCPauseMillis=50",
            "-XX:G1HeapRegionSize=32M",
            "-Dos.name=Windows 10",
            "-Dos.version=10.0",
            "-Dfml.ignorePatchDiscrepancies=true",
            "-Dfml.ignoreInvalidMinecraftCertificates=true",
            "-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump",
            "-Dminecraft.launcher.brand=AtlasLauncher",
            "-Dminecraft.launcher.version=1",
        ])
        .arg(format!("-Djava.library.path={}", check_directory(format!("natives/{version}").as_str()).await.to_str().unwrap()).as_str())
        .arg(format!("-Dminecraft.applet.TargetDirectory={}", check_directory(format!("instances/{name}").as_str()).await.to_str().unwrap()))
        .arg(main_class)
        .args(args)
        //.creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .expect("failed to execute java process");

    let status = process.wait().unwrap().code().unwrap();
    println!("{status}");

}

pub async fn get_instances() -> Vec<InstanceInfo> {
    let mut instances: Vec<InstanceInfo> = Vec::new();

    let instances_path: PathBuf = check_directory("instances").await;

    for entry in fs::read_dir(instances_path).unwrap() {
        let entry: DirEntry = entry.unwrap();
        let path: PathBuf = entry.path();

        let contents: String = fs::read_to_string(&path.join("atlas_instance.json")).unwrap();
        let instance: InstanceInfo = serde_json::from_str(&contents).unwrap();
        instances.push(instance);
    }

    instances
}

pub async fn get_libraries(libraries: &Value) -> String {
    let mut result: String = String::from("");

    let libraries_path: String = String::from(
        env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("libraries")
            .to_str()
            .unwrap(),
    );

    for download in libraries.as_array().unwrap() {
        if let Some(_artifact) = download["downloads"].get("artifact") {
            let mut must_use: bool = true;
            if let Some(rules) = download.get("rules") {
                for rule in rules.as_array().unwrap().iter() {
                    if let Some(action) = rule.get("action").and_then(|a| a.as_str()) {
                        if action == "allow" {
                            if let Some(os) = rule
                                .get("os")
                                .and_then(|os| os.get("name").and_then(|n| n.as_str()))
                            {
                                if os != "windows" {
                                    must_use = false;
                                }
                            }
                        }
                    }
                }
            }
            let path: &str = download["downloads"]["artifact"]["path"]
                .as_str()
                .unwrap_or_default();

            if must_use {
                result = format!("{result}{libraries_path}/{path};");
            }
        }
    }

    result
}
