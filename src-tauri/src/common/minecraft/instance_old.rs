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
    let libraries_arg: String = downloader::download(id).await.unwrap();

    let version_info: Value = file::read_as_value(format!("versions/{id}/{id}.json").as_str())
        .await
        .unwrap();

    let mut java_version: u64 = version_info["javaVersion"]["majorVersion"]
        .as_u64()
        .unwrap();

    java_version = match java_version {
        8 => 8,
        17 => 17,
        _ => 8,
    };

    let game_arguments: Option<&Vec<Value>> = version_info["arguments"]["game"].as_array();
    let mut parsed_game_arguments: Vec<String> = vec![];
    if game_arguments.is_some() {
        for argument in game_arguments.unwrap() {
            if argument.is_string() {
                parsed_game_arguments.push(argument.as_str().unwrap().to_string())
            }
        }
    } else {
        parsed_game_arguments = version_info["minecraftArguments"]
            .as_str()
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
    }

    parsed_game_arguments.append(&mut vec![
        String::from("--width"),
        String::from("1280"),
        String::from("--height"),
        String::from("720"),
    ]);

    let jvm_arguments: Option<&Vec<Value>> = version_info["arguments"]["jvm"].as_array();
    let mut parsed_jvm_arguments: Vec<String> = vec![];

    if jvm_arguments.is_some() {
        for argument in jvm_arguments.unwrap() {
            if argument.is_string() {
                parsed_jvm_arguments.push(argument.as_str().unwrap().to_string())
            }
        }
    } else {
        parsed_jvm_arguments.append(&mut vec![
            String::from("-cp"),
            String::from("${classpath}"),
            String::from("-Djava.library.path=${natives_directory}"),
        ]);
    }

    let asset_index: &str = match version_info["assetIndex"]["id"].as_str() {
        Some(index) => index,
        None => "",
    };
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

    parsed_jvm_arguments.append(&mut jvm_args);

    let instance_info: InstanceInfo = InstanceInfo {
        name: String::from(name),
        version: String::from(id),
        background: String::from(""),
        libraries: String::from(libraries_arg),
        java_version: java_version,
        minecraft_args: parsed_game_arguments,
        asset_index: String::from(asset_index),
        version_type: String::from(version_type),
        main_class: String::from(main_class),
        jvm_args: parsed_jvm_arguments,
    };

    check_directory(format!("instances/{name}/resourcepacks").as_str()).await;

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

    let mut game_args = instance_info.minecraft_args.clone();

    for game_arg in game_args.iter_mut() {
        *game_arg = game_arg
            .replace("${auth_player_name}", &active_user.username)
            .replace("${auth_session}", &active_user.access_token)
            .replace("${game_directory}", format!("{}", &instance_path).as_str())
            .replace("${game_assets}", &assets_path)
            .replace("${version_name}", &instance_info.version)
            .replace("${assets_root}", &assets_path)
            .replace("${assets_index_name}", &instance_info.asset_index)
            .replace("${auth_uuid}", &active_user.uuid)
            .replace("${auth_access_token}", &active_user.access_token)
            .replace("${user_properties}", "{}")
            .replace("${user_type}", "msa")
            .replace("${profile_name}", "Minecraft")
            .replace("${resolution_width}", "1280")
            .replace("${resolution_height}", "720")
            .replace("${version_type}", &instance_info.version_type)
    }

    let mut jvm_args = instance_info.jvm_args.clone();

    for jvm_arg in jvm_args.iter_mut() {
        *jvm_arg = jvm_arg
            .replace(
                "${natives_directory}",
                check_directory(format!("natives/{}", instance_info.version).as_str())
                    .await
                    .to_str()
                    .unwrap(),
            )
            .replace("${auth_uuid}", &active_user.uuid)
            .replace("${profile_name}", "Minecraft")
            .replace(
                "${assets_root}",
                check_directory("assets").await.to_str().unwrap(),
            )
            .replace("${assets_index_name}", &instance_info.asset_index)
            .replace("${classpath}", &cp)
            .replace("${game_directory}", format!("{}", &instance_path).as_str())
    }

    let mut retries: u8 = 0;
    let version: String = instance_info.version.to_owned();

    while {
        retries += 1;
        let mut should_retry: bool = false;

        let mut process: Child = launch(
            &instance_path,
            &java_path,
            &game_args,
            &jvm_args,
            &instance_info.main_class,
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

        version.starts_with("rd-") && retries <= 20 && should_retry
    } {}
}

async fn launch(
    instance_path: &str,
    java_path: &str,
    args: &Vec<String>,
    jvm_args: &Vec<String>,
    main_class: &str,
) -> Child {
    println!("{:?}", args);
    println!("{:?}", jvm_args);
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    let working_dir: PathBuf = env::current_dir().unwrap();
    std::env::set_current_dir(&instance_path).unwrap();

    let process: std::process::Child = Command::new(java_path)
        .args(jvm_args)
        .arg(main_class)
        .args(args)
        // .creation_flags(CREATE_NO_WINDOW)
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
