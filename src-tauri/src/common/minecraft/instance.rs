use serde_json::Value;
use std::{
    env::{
        self,
        consts::{ARCH, OS},
    },
    fs::{self, DirEntry},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    common::auth::login::get_active_account,
    common::{modloader, utils::directory::check_directory_sync},
    common::{
        modloader::util::get_manifest,
        utils::file::{self},
    },
    data::models::{
        BaseEventPayload, DownloadInstanceEventPayload, InstanceInfo, MinecraftAccount,
        StartInstanceEventPayload,
    },
    java::{downloader as javaDownloader, get_java_path::get_java_path},
    minecraft::downloader,
    utils::directory::check_directory,
};

use tauri::{AppHandle, Manager};

use super::downloader::download_libraries;

pub async fn create_instance(id: &str, name: &str, modloader: &str, app: &tauri::AppHandle) {
    let og_id: String = id.to_string();
    let mut id: String = id.to_string();

    app.emit_all(
        "download",
        DownloadInstanceEventPayload {
            base: BaseEventPayload {
                message: String::from("Preparing instance creation"),
                status: String::from("Loading"),
            },
            total: 0,
            downloaded: 0,
            name: name.to_string(),
        },
    )
    .unwrap();

    javaDownloader::download(8, app, name).await.unwrap();
    javaDownloader::download(17, app, name).await.unwrap();

    if modloader.starts_with("forge-") {
        let forge_manifest: Value = modloader::forge::download_manifest(modloader, app, name)
            .await
            .unwrap();
        id = forge_manifest["inheritsFrom"].as_str().unwrap().to_string();
    } else if modloader.starts_with("fabric-") {
        let fabric_manifest: Value = modloader::fabric::download_manifest(&id, modloader)
            .await
            .unwrap();
        id = fabric_manifest["inheritsFrom"]
            .as_str()
            .unwrap()
            .to_string();
    } else if modloader.starts_with("quilt-") {
        let quilt_manifest: Value = modloader::quilt::download_manifest(&id, modloader)
            .await
            .unwrap();
        id = quilt_manifest["inheritsFrom"].as_str().unwrap().to_string();
    }

    downloader::download(&id, app, name).await.unwrap();

    let version_info: Value =
        file::read_as_value(format!("launcher/meta/net.minecraft/{}.json", &id).as_str())
            .await
            .unwrap();

    if modloader.starts_with("forge") {
        modloader::forge::download_forge(&id, &modloader.replace("forge-", ""), name, app)
            .await
            .unwrap();
    } else if modloader.starts_with("fabric") {
        modloader::fabric::download_fabric(&id, &modloader.replace("fabric-", ""), app, &name)
            .await
            .unwrap();
    } else if modloader.starts_with("quilt") {
        modloader::quilt::download_quilt(&id, &modloader.replace("quilt-", ""), app, &name)
            .await
            .unwrap();
    }

    let instance_info: InstanceInfo = InstanceInfo {
        name: String::from(name),
        version: og_id,
        background: String::from("default0"),
        icon: String::from("default0"),
        version_type: version_info["type"].as_str().unwrap().to_string(),
        width: String::from("1920"),
        height: String::from("1080"),
        modloader: String::from(modloader),
        fullscreen: false,
    };

    check_directory(format!("instances/{name}/resourcepacks").as_str()).await;

    file::write_vec(
        &serde_json::to_vec(&instance_info).unwrap(),
        &format!("instances/{name}/atlas_instance.json"),
    )
    .unwrap();

    app.emit_all(
        "download",
        DownloadInstanceEventPayload {
            base: BaseEventPayload {
                message: String::from(""),
                status: String::from("Success"),
            },
            total: 0,
            downloaded: 0,
            name: name.to_string(),
        },
    )
    .unwrap();
}

pub async fn launch_instance(name: &str, app: &tauri::AppHandle) {
    // jsons info
    let instance_info: InstanceInfo =
        file::read_as_value(format!("instances/{name}/atlas_instance.json").as_str())
            .await
            .unwrap();

    let mut modloader_manifest: Option<Value> = None;

    let version_info: Value = file::read_as_value(
        format!(
            "launcher/meta/net.minecraft/{}.json",
            if instance_info.modloader.is_empty() {
                instance_info.version.to_string()
            } else {
                let new_modloader_manifest: Value =
                    get_manifest(&instance_info.modloader, app, name, &instance_info.version)
                        .await
                        .unwrap();
                modloader_manifest = Some(new_modloader_manifest.clone());
                new_modloader_manifest["inheritsFrom"]
                    .as_str()
                    .unwrap()
                    .to_string()
            }
        )
        .as_str(),
    )
    .await
    .unwrap();

    // active user
    let active_user: MinecraftAccount = get_active_account().unwrap();

    // java
    let mut java_version: u64 = version_info["javaVersion"]["majorVersion"]
        .as_u64()
        .unwrap_or(8);

    java_version = match java_version {
        8 => 8,
        16 => 17,
        17 => 17,
        _ => 8,
    };

    let java_path: String = get_java_path(java_version.try_into().unwrap()).await;

    // libraries
    let libraries_path: String = String::from(check_directory("libraries").await.to_str().unwrap());
    let libraries: String = match version_info["libraries"].as_array() {
        Some(libraries) => download_libraries(
            libraries,
            &instance_info.version,
            true,
            app,
            &instance_info.name,
        )
        .await
        .unwrap(),
        None => String::from(""),
    };

    let forge_jar = check_directory(format!("versions").as_str())
        .await
        .join(format!("{}.jar", { &instance_info.modloader }));

    // classpath
    let version_path: String = if instance_info.modloader.is_empty() || !forge_jar.exists() {
        String::from(
            check_directory(format!("versions").as_str())
                .await
                .join(format!("{}.jar", { &instance_info.version }))
                .to_str()
                .unwrap(),
        )
    } else {
        String::from(
            check_directory(format!("versions").as_str())
                .await
                .join(format!("{}.jar", { &instance_info.modloader }))
                .to_str()
                .unwrap(),
        )
    };
    let mut cp: String = format!("{version_path};{libraries}",);

    // args
    let instance_path: String = String::from(
        check_directory(format!("instances/{name}").as_str())
            .await
            .to_str()
            .unwrap(),
    );

    let version_type: &str = version_info["type"].as_str().unwrap();

    let asset_index: &str = match version_info["assetIndex"]["id"].as_str() {
        Some(index) => index,
        None => "",
    };
    let assets_path: String = String::from(
        check_directory(
            format!(
                "{}",
                if asset_index == "pre-1.6" {
                    format!("instances/{name}/resources")
                } else if asset_index == "legacy" {
                    format!("assets/virtual/legacy")
                } else {
                    "assets".to_string()
                }
            )
            .as_str(),
        )
        .await
        .to_str()
        .unwrap(),
    );

    let game_arguments: Option<&Vec<Value>> = version_info["arguments"]["game"].as_array();
    let mut parsed_game_arguments: Vec<String> = vec![];
    if game_arguments.is_some() {
        for argument in game_arguments.unwrap() {
            if argument.is_string() {
                parsed_game_arguments.push(argument.as_str().unwrap().to_string())
            };
        }
    } else if let Some(minecraft_arguments) = version_info["minecraftArguments"].as_str() {
        parsed_game_arguments = minecraft_arguments
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
    }

    parsed_game_arguments.append(&mut vec![
        String::from("--width"),
        String::from("${resolution_width}"),
        String::from("--height"),
        String::from("${resolution_height}"),
    ]);

    let jvm_arguments: Option<&Vec<Value>> = version_info["arguments"]["jvm"].as_array();
    let mut parsed_jvm_arguments: Vec<String> = vec![];

    if jvm_arguments.is_some() {
        for argument in jvm_arguments.unwrap() {
            if argument.is_string() {
                parsed_jvm_arguments.push(argument.as_str().unwrap().to_string())
            } else if let Some(rules) = argument.get("rules") {
                let formatted_os: &str = match OS {
                    "macos" => "osx",
                    _ => OS,
                };
                let formatted_arch: &str = match ARCH {
                    "x86" => "x86",
                    "x86_64" => "x64",
                    _ => "x64",
                };
                let mut must_use: bool = true;
                for rule in rules.as_array().unwrap().iter() {
                    if let Some(action) = rule.get("action").and_then(|a| a.as_str()) {
                        if action == "allow" {
                            if let Some(os) = rule.get("os") {
                                if let Some(name) = os.get("name") {
                                    if name.as_str().unwrap() != formatted_os {
                                        must_use = false;
                                    }
                                }
                                if let Some(arch) = os.get("arch") {
                                    if arch.as_str().unwrap() != formatted_arch {
                                        must_use = false;
                                    }
                                }
                            }
                        }
                    }
                }
                if must_use {
                    if argument["value"].is_array() {
                        for child in argument["value"].as_array().unwrap() {
                            parsed_jvm_arguments.push(child.as_str().unwrap().to_string())
                        }
                    } else {
                        parsed_jvm_arguments.push(argument["value"].as_str().unwrap().to_string())
                    }
                }
            }
        }
    } else {
        parsed_jvm_arguments.append(&mut vec![
            String::from("-cp"),
            String::from("${classpath}"),
            String::from("-Djava.library.path=${natives_directory}"),
        ]);
    }

    parsed_jvm_arguments.append(&mut vec![
        String::from("-Dfml.ignoreInvalidMinecraftCertificates=true"),
        String::from("-Dfml.ignorePatchDiscrepancies=true"),
        String::from("-Dminecraft.applet.TargetDirectory=${game_directory}"),
        String::from("-Xmx2G"),
        String::from("-Xms2G"),
        String::from("-XX:+UnlockExperimentalVMOptions"),
        String::from("-XX:+UseG1GC"),
        String::from("-XX:G1ReservePercent=20"),
    ]);

    if java_version == 17 {
        parsed_jvm_arguments.append(&mut vec![
            String::from("-XX:+UnlockDiagnosticVMOptions"),
            String::from("-XX:+AlwaysActAsServerClassMachine"),
            String::from("-XX:+AlwaysPreTouch"),
            String::from("-XX:+DisableExplicitGC"),
            String::from("-XX:+UseNUMA"),
            String::from("-XX:NmethodSweepActivity=1"),
            String::from("-XX:ReservedCodeCacheSize=400M"),
            String::from("-XX:NonNMethodCodeHeapSize=12M"),
            String::from("-XX:ProfiledCodeHeapSize=194M"),
            String::from("-XX:NonProfiledCodeHeapSize=194M"),
            String::from("-XX:-DontCompileHugeMethods"),
            String::from("-XX:MaxNodeLimit=240000"),
            String::from("-XX:NodeLimitFudgeFactor=8000"),
            String::from("-XX:+UseVectorCmov"),
            String::from("-XX:+PerfDisableSharedMem"),
            String::from("-XX:+UseFastUnorderedTimeStamps"),
            String::from("-XX:+UseCriticalJavaThreadPriority"),
            String::from("-XX:ThreadPriorityPolicy=1"),
            String::from("-XX:AllocatePrefetchStyle=3"),
            String::from("-XX:MaxGCPauseMillis=37"),
            String::from("-XX:+PerfDisableSharedMem"),
            String::from("-XX:G1HeapRegionSize=16M"),
            String::from("-XX:G1NewSizePercent=23"),
            String::from("-XX:SurvivorRatio=32"),
            String::from("-XX:G1MixedGCCountTarget=3"),
            String::from("-XX:G1HeapWastePercent=20"),
            String::from("-XX:InitiatingHeapOccupancyPercent=10"),
            String::from("-XX:G1RSetUpdatingPauseTimePercent=0"),
            String::from("-XX:MaxTenuringThreshold=1"),
            String::from("-XX:G1SATBBufferEnqueueingThresholdPercent=30"),
            String::from("-XX:G1ConcMarkStepDurationMillis=5.0"),
            String::from("-XX:G1ConcRSHotCardLimit=16"),
            String::from("-XX:G1ConcRefinementServiceIntervalMillis=150"),
            String::from("-XX:GCTimeRatio=99"),
        ]);
    } else {
        parsed_jvm_arguments.append(&mut vec![
            String::from("-XX:G1NewSizePercent=20"),
            String::from("-XX:MaxGCPauseMillis=50"),
            String::from("-XX:G1HeapRegionSize=32M"),
        ]);
    }

    let mut main_class: String = version_info["mainClass"].as_str().unwrap().to_string();

    if modloader_manifest.is_some() {
        let modloader_manifest: Value = modloader_manifest.unwrap();
        if let Some(modloader_arguments) = modloader_manifest["arguments"]["game"].as_array() {
            for argument in modloader_arguments {
                parsed_game_arguments.push(argument.as_str().unwrap().to_string());
            }
        }
        if let Some(modloader_jvm_argument) = modloader_manifest["arguments"]["jvm"].as_array() {
            for argument in modloader_jvm_argument {
                parsed_jvm_arguments.push(argument.as_str().unwrap().to_string());
            }
        }
        if let Some(libraries) = modloader_manifest["libraries"].as_array() {
            let modloader_libraries: String = download_libraries(
                libraries,
                &instance_info.version,
                true,
                app,
                &instance_info.name,
            )
            .await
            .unwrap();
            cp = format!("{cp};{modloader_libraries}");
        }

        if let Some(mc) = modloader_manifest["mainClass"].as_str() {
            main_class = mc.to_string();
        }
        if let Some(cp_ignore) = modloader_manifest["arguments"]["cp_ignore"].as_array() {
            for ignore in cp_ignore {
                cp = cp.replace(ignore.as_str().unwrap(), "");
            }
        }
    }

    if OS == "windows" {
        cp = cp.replace("/", "\\");
    }

    cp = cp
        .replace("${libraries_path}", &libraries_path)
        .replace(";;", ";");

    for game_arg in parsed_game_arguments.iter_mut() {
        *game_arg = game_arg
            .replace("${auth_player_name}", &active_user.username)
            .replace("${auth_session}", &active_user.access_token)
            .replace("${game_directory}", format!("{}", &instance_path).as_str())
            .replace("${game_assets}", &assets_path)
            .replace("${version_name}", &instance_info.version)
            .replace("${assets_root}", &assets_path)
            .replace("${assets_index_name}", &asset_index)
            .replace("${auth_uuid}", &active_user.uuid)
            .replace("${auth_access_token}", &active_user.access_token)
            .replace("${user_properties}", "{}")
            .replace("${user_type}", "msa")
            .replace("${profile_name}", "Minecraft")
            .replace("${resolution_width}", &instance_info.width)
            .replace("${resolution_height}", &instance_info.height)
            .replace("${version_type}", version_type);

        if OS == "windows" {
            *game_arg = game_arg.replace("/", "\\");
        };
    }

    if instance_info.fullscreen {
        parsed_game_arguments.push(String::from("--fullscreen"));
        parsed_game_arguments.push(String::from("true"));
    }

    for jvm_arg in parsed_jvm_arguments.iter_mut() {
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
            .replace("${version_name}", &instance_info.version)
            .replace("${assets_index_name}", &asset_index)
            .replace("${classpath}", &cp)
            .replace("${libraries_path}", &libraries_path)
            .replace("${library_directory}", &libraries_path)
            .replace("${classpath_separator}", ";")
            .replace("${game_directory}", format!("{}", &instance_path).as_str());

        if OS == "windows" && jvm_arg.matches('/').count() > 1 {
            *jvm_arg = jvm_arg.replace("/", "\\");
        };
    }

    // logging
    if let Some(logging) = version_info.get("logging") {
        if let Some(client) = logging.get("client") {
            let mut id: &str = "";
            if let Some(file) = client.get("file") {
                id = file["id"].as_str().unwrap();
            }
            if let Some(argument) = client.get("argument") {
                parsed_jvm_arguments.push(
                    argument.as_str().unwrap().replace(
                        "${path}",
                        &(check_directory("assets\\log_configs")
                            .await
                            .to_str()
                            .unwrap()
                            .to_string()
                            + "\\"
                            + id),
                    ),
                );
            }
        }
    }

    let mut retries: u8 = 0;

    while {
        retries += 1;
        let mut should_retry: bool = false;

        let mut process: Child = launch(
            &instance_path,
            &java_path,
            &parsed_game_arguments,
            &parsed_jvm_arguments,
            &main_class,
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

        instance_info.version.starts_with("rd-") && retries <= 20 && should_retry
    } {}
}

async fn launch(
    instance_path: &str,
    java_path: &str,
    args: &Vec<String>,
    jvm_args: &Vec<String>,
    main_class: &str,
) -> Child {
    println!("{:?}", jvm_args);
    println!("{main_class}");
    println!("{:?}", args);
    let working_dir: PathBuf = env::current_dir().unwrap();
    std::env::set_current_dir(&instance_path).unwrap();

    let process: std::process::Child = Command::new(java_path)
        .args(jvm_args)
        .arg(main_class)
        .args(args)
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
        if !instance.background.starts_with("default") {
            instance.background = path.join(instance.background).to_str().unwrap().to_string();
        }
        if !instance.icon.starts_with("default") {
            instance.icon = path.join(instance.icon).to_str().unwrap().to_string();
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

    if !instance.background.starts_with("default") {
        instance.background = path.join(instance.background).to_str().unwrap().to_string();
    }
    if !instance.icon.starts_with("default") {
        instance.icon = path.join(instance.icon).to_str().unwrap().to_string();
    }
    instance
}

pub async fn write_instance(name: &str, data: InstanceInfo, app: &AppHandle) {
    let instances_path: PathBuf = check_directory_sync(format!("instances").as_str());
    let old_instance_path: PathBuf = instances_path.join(name);
    let new_instance_path: PathBuf = instances_path.join(&data.name);

    if name != data.name {
        fs::rename(old_instance_path, &new_instance_path).unwrap();
    }
    let atlas_instance_path: PathBuf = new_instance_path.join("atlas_instance.json");
    let contents: String = fs::read_to_string(&atlas_instance_path).unwrap();

    let mut instance: InstanceInfo = serde_json::from_str(&contents).unwrap();
    instance.name = data.name.to_string();

    let now: SystemTime = SystemTime::now();
    let since_epoch: std::time::Duration = now.duration_since(UNIX_EPOCH).unwrap();
    let timestamp: String = since_epoch.as_secs().to_string();

    if !data.background.starts_with("default") {
        let background_path: &Path = Path::new(&data.background);
        let bg_name: &str = background_path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap();

        if bg_name != instance.background {
            let extension: &str = &background_path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or("");
            let background_name: String = timestamp.clone() + "-background." + extension;
            if !instance.background.starts_with("default") {
                fs::remove_file(new_instance_path.join(instance.background)).unwrap();
            }
            instance.background = background_name.clone();
            fs::copy(data.background, new_instance_path.join(background_name)).unwrap();
        }
    } else {
        if !instance.background.starts_with("default") {
            fs::remove_file(new_instance_path.join(instance.background)).unwrap();
        }
        instance.background = data.background.to_string();
    }

    if !data.icon.starts_with("default") {
        let icon_path: &Path = Path::new(&data.icon);
        let ic_name: &str = icon_path.file_name().unwrap_or_default().to_str().unwrap();

        if ic_name != instance.icon {
            let extension: &str = &icon_path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or("");
            let icon_name: String = timestamp.clone() + "-icon." + extension;
            if !instance.icon.starts_with("default") {
                fs::remove_file(new_instance_path.join(instance.icon)).unwrap();
            }
            instance.icon = icon_name.clone();
            fs::copy(data.icon, new_instance_path.join(icon_name)).unwrap();
        }
    } else {
        if !instance.icon.starts_with("default") {
            fs::remove_file(new_instance_path.join(instance.icon)).unwrap();
        }
        instance.icon = data.icon.to_string();
    }

    if instance.version != data.version {
        downloader::download(&data.version, app, &instance.name)
            .await
            .unwrap();
        instance.version = data.version;
        instance.version_type = data.version_type;
    }

    instance.height = data.height;
    instance.width = data.width;
    instance.fullscreen = data.fullscreen;

    file::write_value(
        &instance,
        format!("instances/{}/atlas_instance.json", data.name).as_str(),
    )
    .unwrap();
}
