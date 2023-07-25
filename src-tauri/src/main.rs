// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod data;
mod minecraft;
mod modloader;
mod util;

use auth::msauth;
use minecraft::versions;
use std::sync::OnceLock;
use tauri::{async_runtime, AppHandle};

static APP: OnceLock<AppHandle> = OnceLock::new();

#[tokio::main]
async fn main() {
    // to avoid problems due to having multiple async runtimes running
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            msauth::start_msauth,
            msauth::get_accounts,
            msauth::set_active_account,
            msauth::remove_account,
            versions::update_minecraft_version_manifest,
        ])
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
        }))
        .setup(|app| {
            APP.set(app.handle()).unwrap();

            // refresh ms tokens
            async_runtime::spawn(async move {
                msauth::refresh_bearer_tokens().await.unwrap_or_default();
            });
            // update the version manifest
            /* tauri::async_runtime::spawn(async move {
                match minecraft::versions::download_version_manifests().await {
                    Ok(_) => println!("Version manifest successfully updated"),
                    Err(err) => {
                        write_line(&format!("Error updating manifest: {:?}", err));
                    }
                }
            });*/

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
