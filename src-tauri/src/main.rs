// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod data;
mod managers;
mod minecraft;
mod models;
mod util;

use auth::msauth;
use managers::{account::AccountManager, instance::InstanceManager, metadata::MetadataManager};
use models::metadata::MinecraftVersion;
use std::{env, path::PathBuf, sync::OnceLock};
use tauri::{async_runtime, AppHandle, Manager, State};

static APP: OnceLock<AppHandle> = OnceLock::new();
static APP_DIRECTORY: OnceLock<PathBuf> = OnceLock::new();

#[tauri::command]
async fn get_minecraft_versions(
    metadata_manager: State<'_, MetadataManager>,
) -> Result<Vec<MinecraftVersion>, &'static str> {
    metadata_manager.get_minecraft_versions().await
}

#[tauri::command]
async fn create_instance(
    instance_manager: State<'_, InstanceManager>,
    name: &str,
    version: &str,
) -> Result<(), &'static str> {
    instance_manager.create_instance(name, version).await
}

#[tokio::main]
async fn main() {
    // to avoid problems due to having multiple async runtimes running
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    APP_DIRECTORY
        .set(env::current_exe().unwrap().parent().unwrap().to_path_buf())
        .expect("Error setting up app directory");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            msauth::start_msauth,
            msauth::get_accounts,
            msauth::set_active_account,
            msauth::remove_account,
            get_minecraft_versions,
            create_instance
        ])
        .manage(AccountManager::init().await)
        .manage(MetadataManager::init().await)
        .manage(InstanceManager::init().await)
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
        }))
        .setup(|app| {
            APP.set(app.handle()).unwrap();

            // Refresh accounts info
            async_runtime::spawn(async move {
                let manager = APP.get().unwrap().state::<AccountManager>();
                manager.refresh_accounts().await.unwrap_or_default();
            });

            // Refresh metadata
            async_runtime::spawn(async move {
                let manager = APP.get().unwrap().state::<MetadataManager>();
                manager.refesh_manifests().await.unwrap_or_default();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error running tauri app");
}
