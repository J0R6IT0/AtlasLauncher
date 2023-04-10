// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, fs, path::PathBuf};
use tauri::AppHandle;
use tokio;

mod common;
use common::auth;
use common::minecraft;
use common::utils;


#[tauri::command]
fn list_minecraft_versions(version_type: &str) -> Vec<String> {
    match minecraft::versions::get_versions(version_type) {
        Ok(version_list) => version_list.iter().map(|v| v.id.clone()).collect(),
        Err(_) => {
            [].to_vec()
        }
    }
}

#[tauri::command]
async fn start_oauth(handle: tauri::AppHandle) {
    auth::login::create_login_window(handle);
}

#[tauri::command]
fn get_accounts() -> Vec<auth::login::AccountInfo> {
    auth::login::get_accounts()
}

#[tauri::command]
fn get_active_account() -> String {
    auth::login::get_active_account()
}

#[tauri::command]
fn set_active_account(uuid: &str) {
    auth::login::set_active_account(uuid);
}

#[tauri::command]
fn remove_account(uuid: &str) {
    auth::login::remove_account(uuid);
}

#[tauri::command]
async fn create_instance(version_type: &str, version: &str, name: &str) -> Result<(), ()> {
    minecraft::instance::create_instance(version_type, version, name).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Checks and creates missing folders
    check_folder_structure();
    // Updates the version manifest
    match minecraft::versions::download_version_manifest().await {
        Ok(_) => println!("Version manifest successfully updated"),
        Err(err) => println!("Error updating manifest: {:?}", err),
    }

    // minecraft::instance::launch_instance("De chill").await;

    tauri::async_runtime::set(tokio::runtime::Handle::current());
    // Builds the Tauri app
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_minecraft_versions,
            start_oauth,
            get_accounts,
            get_active_account,
            set_active_account,
            remove_account,
            create_instance,
        ])
        .setup(|app| {
            let handle: AppHandle = app.handle();
            tauri::async_runtime::spawn(async move {
                auth::bearer_token::refresh_bearer_tokens(&handle).await;
            });
            Ok(())

        })
        .run({
            tauri::generate_context!()
        })
        .expect("error while running tauri application");
}

fn check_folder_structure() {
    // Main launcher stuff is located inside /launcher
    let launcher_path: PathBuf = env::current_exe()
        .expect("Failed to get executable path")
        .parent()
        .unwrap()
        .join("launcher");

    ["version-info", "auth"].iter().for_each(|subdir| {
        let path: PathBuf = launcher_path.join(subdir);
        if !path.exists() {
            fs::create_dir_all(&path).expect("Failed to create directory");
        }
    });
}
