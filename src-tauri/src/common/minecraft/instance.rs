use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    env,
    fs::{self, DirEntry},
    io::{BufRead, BufReader},
    os::windows::process::CommandExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

use regex::Regex;

use crate::{common::auth::login::get_active_account, minecraft::downloader, data::models::MinecraftAccount};
use crate::{common::utils::directory::check_directory_sync, utils::directory::check_directory};
use crate::{
    common::utils::file,
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

#[derive(Clone, Serialize)]
pub struct StartInstanceEventPayload {
    pub message: String,
    pub status: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InstanceInfo {
    pub name: String,
    pub version: String,
    pub libraries: String,
    pub background: String,
}

pub async fn create_instance(
    id: &str,
    name: &str,
    app: &tauri::AppHandle,
) {
    check_directory(format!("instances/{name}/resourcepacks").as_str()).await;

    let instance_info: String = format!(
        r#"
            {{
                "name": "{name}",
                "version": "{id}"
            }}
        "#
    );

    file::write_str(
        &instance_info,
        &format!("instances/{name}/atlas_instance.json"),
    )
    .unwrap();

    app.emit_all(
        "create_instance",
        CreateInstanceEventPayload {
            version: String::from(id),
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
            version: String::from(id),
            name: String::from(name),
            message: format!("Downloading game files"),
            status: String::from("Loading"),
        },
    )
    .unwrap();

    let libraries_arg = downloader::download(id, name)
        .await
        .unwrap();

    let instance_info: String = format!(
        r#"
            {{
                "name": "{name}",
                "version": "{id}",
                "background": "",
                "libraries": "{libraries_arg}"
            }}
        "#
    );

    file::write_str(
        &instance_info,
        &format!("instances/{name}/atlas_instance.json"),
    )
    .unwrap();

    app.emit_all(
        "create_instance",
        CreateInstanceEventPayload {
            version: String::from(id),
            name: String::from(name),
            message: format!("Instance created successfully"),
            status: String::from("Success"),
        },
    )
    .unwrap();
}

pub async fn launch_instance(name: &str, app: &tauri::AppHandle) {
    // instance info
    let instance_info: Value =
        file::read_as_json(format!("instances/{name}/atlas_instance.json").as_str())
            .await
            .unwrap();

    // version info
    let version: &str = instance_info["version"].as_str().unwrap();
    let version_info: Value =
        file::read_as_json(format!("versions/{version}/{version}.json").as_str())
            .await
            .unwrap();

    // java
    let java_version: u64 = match version_info["javaVersion"]["majorVersion"].as_u64() {
        Some(java_version) => match java_version {
            8 => 8,
            17 => 17,
            _ => 17,
        },
        None => 8,
    };

    let java_path: String = get_java_path(java_version.try_into().unwrap()).await;

    // paths
    let version_path: String = String::from(
        check_directory(format!("versions/{version}").as_str())
            .await
            .join(format!("{version}.jar"))
            .to_str()
            .unwrap(),
    );
    let instance_path: String = String::from(
        check_directory(format!("instances/{name}").as_str())
            .await
            .to_str()
            .unwrap(),
    );
    let libraries_path: String = String::from(check_directory("libraries").await.to_str().unwrap());
    let libraries: String = instance_info["libraries"]
        .as_str()
        .unwrap()
        .replace("[libraries_path]", &libraries_path);
    let asset_index: &str = version_info["assetIndex"]["id"].as_str().unwrap();
    let assets_path: String = String::from(
        check_directory(
            format!(
                "{}",
                if asset_index == "legacy" || asset_index == "pre-1.6" {
                    "assets/virtual/legacy"
                } else {
                    "assets"
                }
            )
            .as_str(),
        )
        .await
        .to_str()
        .unwrap(),
    );

    let cp: String = format!("{version_path};{libraries}");

    let active_user: MinecraftAccount = get_active_account().unwrap();

    let main_class: &str = version_info["mainClass"].as_str().unwrap();

    let arguments = match version_info["minecraftArguments"].as_str() {
        Some(arguments) => arguments,
        None => "--username ${auth_player_name} --version ${version_name} --gameDir ${game_directory} --assetsDir ${assets_root} --assetIndex ${assets_index_name} --uuid ${auth_uuid} --accessToken ${auth_access_token} --userType ${user_type}"
    };
    let replaced_arguments = arguments
        .replace(
            "${auth_player_name}",
            &active_user.username,
        )
        .replace(
            "${auth_session}",
            &active_user.access_token,
        )
        .replace(
            "${game_directory}",
            format!("\"{}\"", &instance_path).as_str(),
        )
        .replace("${game_assets}", &assets_path)
        .replace("${version_name}", version)
        .replace("${assets_root}", &assets_path)
        .replace("${assets_index_name}", asset_index)
        .replace("${auth_uuid}", &active_user.uuid)
        .replace(
            "${auth_access_token}",
            &active_user.access_token,
        )
        .replace("${user_properties}", "{}")
        .replace("${user_type}", "msa")
        .replace("${version_type}", version_info["type"].as_str().unwrap());

    let re: Regex = Regex::new(r#""([^"]+)"|(\S+)"#).unwrap();

    let args: Vec<&str> = re
        .captures_iter(&replaced_arguments)
        .map(|cap| {
            if let Some(s) = cap.get(1) {
                s.as_str()
            } else {
                cap.get(2).unwrap().as_str()
            }
        })
        .collect();

    const CREATE_NO_WINDOW: u32 = 0x08000000;

    // we change the working dir to avoid old versions creating files out of their instance folder
    let working_dir: PathBuf = env::current_dir().unwrap();
    std::env::set_current_dir(&instance_path).unwrap();

    let mut jvm_args: Vec<&str> = [
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
    ]
    .to_vec();

    let logging_arg: String;
    if let Some(logging) = version_info.get("logging") {
        if let Some(client) = logging.get("client") {
            let argument: String = String::from(client["argument"].as_str().unwrap());
            if let Some(file) = client.get("file") {
                let id: &str = file["id"].as_str().unwrap();
                logging_arg = argument.to_owned().replace(
                    "${path}",
                    check_directory("assets/log_configs")
                        .await
                        .join(id)
                        .to_str()
                        .unwrap(),
                );
                jvm_args.push(&logging_arg);
            }
        }
    }

    let mut process = Command::new(java_path)
        .arg("-cp")
        .arg(cp)
        .args(jvm_args)
        .arg(
            format!(
                "-Djava.library.path={}",
                check_directory(format!("natives/{version}").as_str())
                    .await
                    .to_str()
                    .unwrap()
            )
            .as_str(),
        )
        .arg(format!(
            "-Dminecraft.applet.TargetDirectory={}",
            &instance_path
        ))
        .arg(main_class)
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute java process");

    std::env::set_current_dir(working_dir).unwrap();

    let output = process.stdout.as_mut().unwrap();
    let reader = BufReader::new(output);
    let lines = reader.lines();

    let mut first_line_printed: bool = false;

    for line in lines {
        if !first_line_printed {
            first_line_printed = true;
            app.emit_all(
                "start_instance",
                StartInstanceEventPayload {
                    message: format!("Successfully launched {name}"),
                    status: String::from("Success"),
                },
            )
            .unwrap();
        }
        let line: String = line.unwrap();
        println!("{line}");
    }

    let status: i32 = process.wait().unwrap().code().unwrap();
    println!("{status}");
}

pub async fn get_instances() -> Vec<InstanceInfo> {
    let mut instances: Vec<InstanceInfo> = Vec::new();

    let instances_path: PathBuf = check_directory("instances").await;

    for entry in fs::read_dir(instances_path).unwrap() {
        let entry: DirEntry = entry.unwrap();
        let path: PathBuf = entry.path();

        let contents: String = fs::read_to_string(&path.join("atlas_instance.json")).unwrap();
        let mut instance: InstanceInfo = serde_json::from_str(&contents).unwrap();
        if !instance.background.is_empty() {
            instance.background = path.join(instance.background).to_str().unwrap().to_string();
        }
        instances.push(instance);
    }

    instances
}

pub fn remove_instance(name: &str) {
    fs::remove_dir_all(check_directory_sync(format!("instances/{name}").as_str())).unwrap();
}

pub fn open_folder(name: &str) {
    let path: String = check_directory_sync(format!("instances/{name}").as_str())
        .to_str()
        .unwrap()
        .to_owned()
        .replace("/", "\\");

    Command::new("explorer").arg(path).spawn().unwrap();
}

pub fn read_instance(name: &str) -> InstanceInfo {
    let mut path: PathBuf =
        check_directory_sync(format!("instances/{name}").as_str()).join("atlas_instance.json");

    let contents: String = fs::read_to_string(&path).unwrap();
    let mut instance: InstanceInfo = serde_json::from_str(&contents).unwrap();
    path.pop();
    if !instance.background.is_empty() {
        instance.background = path.join(instance.background).to_str().unwrap().to_string();
    }
    instance
}

pub fn write_instance(name: &str, new_name: &str, _version: &str, background: &str) {
    let instances_path: PathBuf = check_directory_sync(format!("instances").as_str());
    let old_instance_path: PathBuf = instances_path.join(name);
    let new_instance_path: PathBuf = instances_path.join(new_name);

    if name != new_name {
        fs::rename(old_instance_path, &new_instance_path).unwrap();
    }
    let atlas_instance_path: PathBuf = new_instance_path.join("atlas_instance.json");
    let contents: String = fs::read_to_string(&atlas_instance_path).unwrap();

    let mut instance: InstanceInfo = serde_json::from_str(&contents).unwrap();
    instance.name = new_name.to_string();

    if !background.is_empty() {
        let now: SystemTime = SystemTime::now();
        let since_epoch: std::time::Duration = now.duration_since(UNIX_EPOCH).unwrap();
        let timestamp: String = since_epoch.as_secs().to_string();
        let extension: &str = Path::new(background)
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("");
        let background_name: String = timestamp + "-background." + extension;
        if !instance.background.is_empty() {
            fs::remove_file(new_instance_path.join(instance.background)).unwrap();
        }
        instance.background = background_name.clone();
        fs::copy(background, new_instance_path.join(background_name)).unwrap();
    }

    let serialized_instance: String = serde_json::to_string(&instance).unwrap();

    file::write_str(
        &serialized_instance,
        format!("instances/{new_name}/atlas_instance.json").as_str(),
    )
    .unwrap();
}
