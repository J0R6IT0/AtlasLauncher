use serde_json::Value;
use std::{
    env,
    fs::{self, DirEntry},
    io::{BufRead, BufReader},
    os::windows::process::CommandExt,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

use regex::Regex;

use crate::{
    common::auth::login::get_active_account,
    common::utils::directory::check_directory_sync,
    common::utils::file,
    data::models::{
        BaseEventPayload, CreateInstanceEventPayload, InstanceInfo, MinecraftAccount,
        StartInstanceEventPayload,
    },
    java::{downloader as javaDownloader, get_java_path::get_java_path},
    minecraft::downloader,
    utils::directory::check_directory,
};

use tauri::Manager;

pub async fn create_instance(id: &str, name: &str, app: &tauri::AppHandle) {
    check_directory(format!("instances/{name}/resourcepacks").as_str()).await;

    app.emit_all(
        "create_instance",
        CreateInstanceEventPayload {
            base: BaseEventPayload {
                message: format!("Downloading Java"),
                status: String::from("Loading"),
            },
            name: String::from(name),
        },
    )
    .unwrap();

    javaDownloader::download(8).await.unwrap();
    javaDownloader::download(17).await.unwrap();

    app.emit_all(
        "create_instance",
        CreateInstanceEventPayload {
            base: BaseEventPayload {
                message: format!("Downloading game files"),
                status: String::from("Loading"),
            },
            name: String::from(name),
        },
    )
    .unwrap();

    // create atlas_instance.json
    let libraries_arg: String = downloader::download(id, name).await.unwrap();

    let version_info: Value = file::read_as_value(format!("versions/{id}/{id}.json").as_str())
        .await
        .unwrap();

    let java_version: u64 = match version_info["javaVersion"]["majorVersion"].as_u64() {
        Some(java_version) => match java_version {
            8 => 8,
            17 => 17,
            _ => 17,
        },
        None => 8,
    };

    let arguments: &str = match version_info["minecraftArguments"].as_str() {
        Some(arguments) => arguments,
        None => "--username ${auth_player_name} --version ${version_name} --gameDir ${game_directory} --assetsDir ${assets_root} --assetIndex ${assets_index_name} --uuid ${auth_uuid} --accessToken ${auth_access_token} --userType ${user_type}"
    };
    let asset_index: &str = version_info["assetIndex"]["id"].as_str().unwrap();
    let version_type: &str = version_info["type"].as_str().unwrap();
    let main_class: &str = version_info["mainClass"].as_str().unwrap();

    let mut jvm_args: Vec<String> = vec![
        String::from("-Xmx2G"),
        String::from("-Xms2G"),
        String::from("-XX:+UnlockExperimentalVMOptions"),
        String::from("-XX:+UseG1GC"),
        String::from("-XX:G1NewSizePercent=20"),
        String::from("-XX:G1ReservePercent=20"),
        String::from("-XX:MaxGCPauseMillis=50"),
        String::from("-XX:G1HeapRegionSize=32M"),
        String::from("-Dos.name=Windows 10"),
        String::from("-Dos.version=10.0"),
        String::from("-Dfml.ignorePatchDiscrepancies=true"),
        String::from("-Dfml.ignoreInvalidMinecraftCertificates=true"),
        String::from("-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump"),
        String::from("-Dminecraft.launcher.brand=AtlasLauncher"),
        String::from("-Dminecraft.launcher.version=1"),
    ];

    let logging_arg: String;
    if let Some(logging) = version_info.get("logging") {
        if let Some(client) = logging.get("client") {
            let argument: String = String::from(client["argument"].as_str().unwrap());
            if let Some(file) = client.get("file") {
                let id: &str = file["id"].as_str().unwrap();
                logging_arg = argument
                    .to_owned()
                    .replace("${path}", format!("[log_configs]\\{}", { id }).as_str());
                jvm_args.push(logging_arg);
            }
        }
    }

    let instance_info: InstanceInfo = InstanceInfo {
        name: String::from(name),
        version: String::from(id),
        background: String::from(""),
        libraries: String::from(libraries_arg),
        java_version: java_version,
        minecraft_args: String::from(arguments),
        asset_index: String::from(asset_index),
        version_type: String::from(version_type),
        main_class: String::from(main_class),
        jvm_args: jvm_args,
    };

    file::write_vec(
        &serde_json::to_vec(&instance_info).unwrap(),
        &format!("instances/{name}/atlas_instance.json"),
    )
    .unwrap();

    app.emit_all(
        "create_instance",
        CreateInstanceEventPayload {
            base: BaseEventPayload {
                message: format!("Instance created successfully"),
                status: String::from("Success"),
            },
            name: String::from(name),
        },
    )
    .unwrap();
}

pub async fn launch_instance(name: &str, app: &tauri::AppHandle) {
    // instance info
    let mut instance_info: InstanceInfo =
        file::read_as_value(format!("instances/{name}/atlas_instance.json").as_str())
            .await
            .unwrap();
    let active_user: MinecraftAccount = get_active_account().unwrap();

    // paths
    let java_path: String = get_java_path(instance_info.java_version.try_into().unwrap()).await;
    let version_path: String = String::from(
        check_directory(format!("versions/{}", { &instance_info.version }).as_str())
            .await
            .join(format!("{}.jar", { &instance_info.version }))
            .to_str()
            .unwrap(),
    );
    let libraries_path: String = String::from(check_directory("libraries").await.to_str().unwrap());
    let libraries: String = instance_info
        .libraries
        .replace("[libraries_path]", &libraries_path);

    let instance_path: String = String::from(
        check_directory(format!("instances/{name}").as_str())
            .await
            .to_str()
            .unwrap(),
    );
    let assets_path: String = String::from(
        check_directory(
            format!(
                "{}",
                if &instance_info.asset_index == "legacy" || &instance_info.asset_index == "pre-1.6"
                {
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

    // args
    let cp: String = format!("{version_path};{libraries}");

    for arg in instance_info.jvm_args.iter_mut() {
        if arg.contains("[log_configs]") {
            *arg = arg
                .replace(
                    "[log_configs]",
                    check_directory_sync("assets/log_configs").to_str().unwrap(),
                )
                .to_string();
        }
    }

    let replaced_arguments: String = instance_info
        .minecraft_args
        .replace("${auth_player_name}", &active_user.username)
        .replace("${auth_session}", &active_user.access_token)
        .replace(
            "${game_directory}",
            format!("\"{}\"", &instance_path).as_str(),
        )
        .replace("${game_assets}", &assets_path)
        .replace("${version_name}", &instance_info.version)
        .replace("${assets_root}", &assets_path)
        .replace("${assets_index_name}", &instance_info.asset_index)
        .replace("${auth_uuid}", &active_user.uuid)
        .replace("${auth_access_token}", &active_user.access_token)
        .replace("${user_properties}", "{}")
        .replace("${user_type}", "msa")
        .replace("${version_type}", &instance_info.version_type);

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

    let mut retries: u8 = 0;
    let version: String = instance_info.version.to_owned();

    while {
        retries += 1;
        let mut should_retry: bool = false;

        let mut process: Child = launch(
            &instance_path,
            &java_path,
            &cp,
            instance_info.to_owned(),
            &args,
        )
        .await;

        let output: &mut std::process::ChildStdout = process.stdout.as_mut().unwrap();
        let reader: BufReader<&mut std::process::ChildStdout> = BufReader::new(output);
        let lines: std::io::Lines<BufReader<&mut std::process::ChildStdout>> = reader.lines();

        let mut first_line_printed: bool = false;

        for line in lines {
            if !first_line_printed {
                first_line_printed = true;
                app.emit_all(
                    "start_instance",
                    StartInstanceEventPayload {
                        base: BaseEventPayload {
                            message: format!("Successfully launched {name}"),
                            status: String::from("Success"),
                        },
                    },
                )
                .unwrap();
            }
            let line: String = line.unwrap();
            println!("{line}");
        }

        let status: i32 = process.wait().unwrap().code().unwrap();
        if !first_line_printed {
            should_retry = true
        };
        println!("{status}");

        version.starts_with("rd-") && retries <= 10 && should_retry
    } {}
}

async fn launch(
    instance_path: &str,
    java_path: &str,
    cp: &str,
    instance_info: InstanceInfo,
    args: &Vec<&str>,
) -> Child {
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    let working_dir: PathBuf = env::current_dir().unwrap();
    std::env::set_current_dir(&instance_path).unwrap();

    let process: std::process::Child = Command::new(java_path)
        .arg("-cp")
        .arg(cp)
        .args(instance_info.jvm_args)
        .arg(
            format!(
                "-Djava.library.path={}",
                check_directory(format!("natives/{}", instance_info.version).as_str())
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
        .arg(instance_info.main_class)
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute java process");

    std::env::set_current_dir(working_dir).unwrap();
    process
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

pub async fn read_instance(name: &str) -> InstanceInfo {
    let mut instance: InstanceInfo =
        file::read_as_value(format!("instances/{name}/atlas_instance.json").as_str())
            .await
            .unwrap();
    let path: PathBuf = check_directory_sync(format!("instances/{name}").as_str());

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

    file::write_value(
        &instance,
        format!("instances/{new_name}/atlas_instance.json").as_str(),
    )
    .unwrap();
}
