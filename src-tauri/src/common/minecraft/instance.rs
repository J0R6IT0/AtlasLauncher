use serde::Serialize;
use std::process::Command;
use std::env;

use crate::java::downloader as javaDownloader;
use crate::minecraft::downloader;
use crate::utils::check_directory::check_directory;

use tauri::Manager;

#[derive(Clone, Serialize)]
pub struct CreateInstanceEventPayload {
    pub name: String,
    pub message: String,
    pub status: String,
}

pub async fn create_instance(
    version_type: &str,
    version: &str,
    name: &str,
    app: &tauri::AppHandle,
) {

    check_directory(format!("instances/{name}").as_str()).await;

    app.emit_all(
        "create_instance",
        CreateInstanceEventPayload {
            name: String::from(name),
            message: format!("Downloading Java"),
            status: String::from("Loading"),
        },
    )
    .unwrap();

    javaDownloader::download("17").await.unwrap();

    app.emit_all(
        "create_instance",
        CreateInstanceEventPayload {
            name: String::from(name),
            message: format!("Downloading game files"),
            status: String::from("Loading"),
        },
    )
    .unwrap();

    downloader::download(version_type, version).await.unwrap();

    app.emit_all(
        "create_instance",
        CreateInstanceEventPayload {
            name: String::from(name),
            message: format!("Instance created successfully"),
            status: String::from("Success"),
        },
    )
    .unwrap();

}

pub async fn launch_instance(name: &str) {
    let exe_path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let library_path = format!("{exe_path}/libraries");
    let natives_path = format!("{exe_path}/natives");
    let assets_path = format!("{exe_path}/assets");
    let instance_path = format!("{exe_path}/instances/{name}");
    let java_path = format!("{exe_path}/java/17.0.1+12/bin/java.exe");

    let class_path = format!("{exe_path}/versions/1.19.4/1.19.4.jar");

    let args: Vec<String> = vec![
        format!("{}", "-Xmx2G"),
        format!("-cp"),
        format!("{}", class_path),
        format!("{}", "net.minecraft.client.main.Main"),
        format!("--accessToken"),
        format!("{}", "eyJraWQiOiJhYzg0YSIsImFsZyI6IkhTMjU2In0.eyJ4dWlkIjoiMjUzNTQzMTk5MjM4NzEwMCIsImFnZyI6IkFkdWx0Iiwic3ViIjoiZDVlMGEzYWItYjlkMi00NjgzLTgwMWUtNGEyODQ1ZDRjYjQ4IiwiYXV0aCI6IlhCT1giLCJucyI6ImRlZmF1bHQiLCJyb2xlcyI6W10sImlzcyI6ImF1dGhlbnRpY2F0aW9uIiwicGxhdGZvcm0iOiJVTktOT1dOIiwieXVpZCI6IjIxMmNjZjU3ODg4NDM4MTE3OGVjODU0NmRlODA1Y2FhIiwibmJmIjoxNjc5MTM5NjkzLCJleHAiOjE2NzkyMjYwOTMsImlhdCI6MTY3OTEzOTY5M30.X30GZI250Y1EsHzIBbYjOfyyl88zdsa8GVix7QGuH54"),
        format!("--assetsDir"),
        format!("{}", assets_path),
        format!("--assetsIndex"),
        format!("{}", "1.19"),
        format!("--gameDir"),
        format!("{}", instance_path),
        format!("--userType"),
        format!("{}", "msa"),
        format!("--username"),
        format!("{}", "J0R6IT00"),
        format!("--uuid"),
        format!("{}", "49135ea01a4740d689097eabb5b881ac"),
        format!("--version"),
        format!("{}", "1.19.4"),
        format!("--versionType"),
        format!("{}", "release"),
    ];

    let mut process = Command::new(java_path)
        .arg("-Xmx2G")
        .arg("-XX:+UnlockExperimentalVMOptions")
        .arg("-XX:+UseG1GC")
        .arg("-XX:G1NewSizePercent=20")
        .arg("-XX:G1ReservePercent=20")
        .arg("-XX:MaxGCPauseMillis=50")
        .arg("-XX:G1HeapRegionSize=32M")
        .arg("-cp")
        .arg(format!("{exe_path}/versions/1.19.4/1.19.4.jar;{exe_path}/libraries/net/sf/jopt-simple/jopt-simple/5.0.4/jopt-simple-5.0.4.jar;{exe_path}/libraries/com/mojang/logging/1.1.1/logging-1.1.1.jar;{exe_path}/libraries/com/mojang/blocklist/1.0.10/blocklist-1.0.10.jar;{exe_path}/libraries/com/mojang/patchy/2.2.10/patchy-2.2.10.jar;{exe_path}/libraries/com/github/oshi/oshi-core/6.2.2/oshi-core-6.2.2.jar;{exe_path}/libraries/net/java/dev/jna/jna/5.12.1/jna-5.12.1.jar;{exe_path}/libraries/net/java/dev/jna/jna-platform/5.12.1/jna-platform-5.12.1.jar;{exe_path}/libraries/org/slf4j/slf4j-api/2.0.1/slf4j-api-2.0.1.jar;{exe_path}/libraries/org/apache/logging/log4j/log4j-slf4j18-impl/2.19.0/log4j-slf4j18-impl-2.19.0.jar;{exe_path}/libraries/com/ibm/icu/icu4j/71.1/icu4j-71.1.jar;{exe_path}/libraries/com/mojang/javabridge/1.2.24/javabridge-1.2.24.jar;{exe_path}/libraries/io/netty/netty-common/4.1.82.Final/netty-common-4.1.82.Final.jar;{exe_path}/libraries/io/netty/netty-buffer/4.1.82.Final/netty-buffer-4.1.82.Final.jar;{exe_path}/libraries/io/netty/netty-codec/4.1.82.Final/netty-codec-4.1.82.Final.jar;{exe_path}/libraries/io/netty/netty-handler/4.1.82.Final/netty-handler-4.1.82.Final.jar;{exe_path}/libraries/io/netty/netty-resolver/4.1.82.Final/netty-resolver-4.1.82.Final.jar;{exe_path}/libraries/io/netty/netty-transport/4.1.82.Final/netty-transport-4.1.82.Final.jar;{exe_path}/libraries/io/netty/netty-transport-native-unix-common/4.1.82.Final/netty-transport-native-unix-common-4.1.82.Final.jar;{exe_path}/libraries/io/netty/netty-transport-classes-epoll/4.1.82.Final/netty-transport-classes-epoll-4.1.82.Final.jar;{exe_path}/libraries/com/google/guava/failureaccess/1.0.1/failureaccess-1.0.1.jar;{exe_path}/libraries/com/google/guava/guava/31.1-jre/guava-31.1-jre.jar;{exe_path}/libraries/org/apache/commons/commons-lang3/3.12.0/commons-lang3-3.12.0.jar;{exe_path}/libraries/commons-io/commons-io/2.11.0/commons-io-2.11.0.jar;{exe_path}/libraries/commons-codec/commons-codec/1.15/commons-codec-1.15.jar;{exe_path}/libraries/com/mojang/brigadier/1.0.18/brigadier-1.0.18.jar;{exe_path}/libraries/com/mojang/datafixerupper/6.0.6/datafixerupper-6.0.6.jar;{exe_path}/libraries/com/google/code/gson/gson/2.10/gson-2.10.jar;{exe_path}/libraries/com/mojang/authlib/3.18.38/authlib-3.18.38.jar;{exe_path}/libraries/org/apache/commons/commons-compress/1.21/commons-compress-1.21.jar;{exe_path}/libraries/org/apache/httpcomponents/httpclient/4.5.13/httpclient-4.5.13.jar;{exe_path}/libraries/commons-logging/commons-logging/1.2/commons-logging-1.2.jar;{exe_path}/libraries/org/apache/httpcomponents/httpcore/4.4.15/httpcore-4.4.15.jar;{exe_path}/libraries/it/unimi/dsi/fastutil/8.5.9/fastutil-8.5.9.jar;{exe_path}/libraries/org/apache/logging/log4j/log4j-api/2.19.0/log4j-api-2.19.0.jar;{exe_path}/libraries/org/apache/logging/log4j/log4j-core/2.19.0/log4j-core-2.19.0.jar;{exe_path}/libraries/org/lwjgl/lwjgl/3.3.1/lwjgl-3.3.1.jar;{exe_path}/libraries/org/lwjgl/lwjgl/3.3.1/lwjgl-3.3.1-natives-windows.jar;{exe_path}/libraries/org/lwjgl/lwjgl/3.3.1/lwjgl-3.3.1-natives-windows-x86.jar;{exe_path}/libraries/org/lwjgl/lwjgl-jemalloc/3.3.1/lwjgl-jemalloc-3.3.1.jar;{exe_path}/libraries/org/lwjgl/lwjgl-jemalloc/3.3.1/lwjgl-jemalloc-3.3.1-natives-windows.jar;{exe_path}/libraries/org/lwjgl/lwjgl-jemalloc/3.3.1/lwjgl-jemalloc-3.3.1-natives-windows-x86.jar;{exe_path}/libraries/org/lwjgl/lwjgl-openal/3.3.1/lwjgl-openal-3.3.1.jar;{exe_path}/libraries/org/lwjgl/lwjgl-openal/3.3.1/lwjgl-openal-3.3.1-natives-windows.jar;{exe_path}/libraries/org/lwjgl/lwjgl-openal/3.3.1/lwjgl-openal-3.3.1-natives-windows-x86.jar;{exe_path}/libraries/org/lwjgl/lwjgl-opengl/3.3.1/lwjgl-opengl-3.3.1.jar;{exe_path}/libraries/org/lwjgl/lwjgl-opengl/3.3.1/lwjgl-opengl-3.3.1-natives-windows.jar;{exe_path}/libraries/org/lwjgl/lwjgl-opengl/3.3.1/lwjgl-opengl-3.3.1-natives-windows-x86.jar;{exe_path}/libraries/org/lwjgl/lwjgl-glfw/3.3.1/lwjgl-glfw-3.3.1.jar;{exe_path}/libraries/org/lwjgl/lwjgl-glfw/3.3.1/lwjgl-glfw-3.3.1-natives-windows.jar;{exe_path}/libraries/org/lwjgl/lwjgl-glfw/3.3.1/lwjgl-glfw-3.3.1-natives-windows-x86.jar;{exe_path}/libraries/org/lwjgl/lwjgl-stb/3.3.1/lwjgl-stb-3.3.1.jar;{exe_path}/libraries/org/lwjgl/lwjgl-stb/3.3.1/lwjgl-stb-3.3.1-natives-windows.jar;{exe_path}/libraries/org/lwjgl/lwjgl-stb/3.3.1/lwjgl-stb-3.3.1-natives-windows-x86.jar;{exe_path}/libraries/org/lwjgl/lwjgl-tinyfd/3.3.1/lwjgl-tinyfd-3.3.1.jar;{exe_path}/libraries/org/lwjgl/lwjgl-tinyfd/3.3.1/lwjgl-tinyfd-3.3.1-natives-windows.jar;{exe_path}/libraries/org/lwjgl/lwjgl-tinyfd/3.3.1/lwjgl-tinyfd-3.3.1-natives-windows-x86.jar;{exe_path}/libraries/com/mojang/text2speech/1.13.9/text2speech-1.13.9.jar;{exe_path}/libraries/com/mojang/text2speech/1.13.9/text2speech-1.13.9-natives-windows.jar;{exe_path}/libraries/org/joml/joml/1.10.5/joml-1.10.5.jar"))
        .arg("net.minecraft.client.main.Main")
        .arg("--username")
        .arg("J0R6IT00")
        .arg("--version")
        .arg("1.19.4")
        .arg("--uuid")
        .arg("49135ea01a4740d689097eabb5b881ac")
        .arg("--accessToken")
        .arg("eyJraWQiOiJhYzg0YSIsImFsZyI6IkhTMjU2In0.eyJ4dWlkIjoiMjUzNTQzMTk5MjM4NzEwMCIsImFnZyI6IkFkdWx0Iiwic3ViIjoiZDVlMGEzYWItYjlkMi00NjgzLTgwMWUtNGEyODQ1ZDRjYjQ4IiwiYXV0aCI6IlhCT1giLCJucyI6ImRlZmF1bHQiLCJyb2xlcyI6W10sImlzcyI6ImF1dGhlbnRpY2F0aW9uIiwicGxhdGZvcm0iOiJVTktOT1dOIiwieXVpZCI6IjIxMmNjZjU3ODg4NDM4MTE3OGVjODU0NmRlODA1Y2FhIiwibmJmIjoxNjc5MTM5NjkzLCJleHAiOjE2NzkyMjYwOTMsImlhdCI6MTY3OTEzOTY5M30.X30GZI250Y1EsHzIBbYjOfyyl88zdsa8GVix7QGuH54")
        .arg("--gameDir")
        .arg(instance_path)
        .arg("--assetsDir")
        .arg(assets_path)
        .arg("--assetIndex")
        .arg("3")
        .spawn()
        .expect("failed to execute java process");

    let status = process.wait().unwrap().code().unwrap();
    println!("{status}");
}
