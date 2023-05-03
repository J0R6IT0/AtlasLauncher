// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::utils::log::write_line;
use std::env;
use tauri::AppHandle;

mod common;
mod data;
use common::{auth, java, minecraft, utils};
use data::models::{self, InstanceInfo};

#[tauri::command]
async fn get_minecraft_versions() -> Result<Vec<models::MinecraftVersionData>, ()> {
    match minecraft::versions::get_versions().await {
        Ok(version_list) => Ok(version_list),
        Err(_) => Ok([].to_vec()),
    }
}

#[tauri::command]
async fn start_oauth(handle: tauri::AppHandle) {
    auth::login::create_login_window(handle);
}

#[tauri::command]
fn get_accounts() -> Vec<models::MinecraftAccount> {
    auth::login::get_accounts()
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
async fn get_instances() -> Vec<InstanceInfo> {
    minecraft::instance::get_instances().await
}

#[tauri::command]
async fn create_instance(name: &str, id: &str, handle: tauri::AppHandle) -> Result<(), ()> {
    minecraft::instance::create_instance(id, name, &handle).await;
    Ok(())
}

#[tauri::command]
async fn launch_instance(name: &str, handle: tauri::AppHandle) -> Result<(), ()> {
    minecraft::instance::launch_instance(name, &handle).await;
    Ok(())
}

#[tauri::command]
fn remove_instance(name: &str) {
    minecraft::instance::remove_instance(name);
}

#[tauri::command]
fn open_instance_folder(name: &str) {
    minecraft::instance::open_folder(name);
}

#[tauri::command]
async fn read_instance_data(name: &str) -> Result<InstanceInfo, ()> {
    Ok(minecraft::instance::read_instance(name).await)
}

#[tauri::command]
async fn write_instance_data(name: &str, data: InstanceInfo, handle: tauri::AppHandle) -> Result<(), ()> {
    minecraft::instance::write_instance(name, data, &handle).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    // to avoid problems due to having multiple async runtimes running
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    // update the version manifest
    match minecraft::versions::download_version_manifest().await {
        Ok(_) => println!("Version manifest successfully updated"),
        Err(err) => {
            println!("Error updating manifest: {:?}", err);
            write_line(&err.to_string());
        }
    }

    // build the tauri app
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_minecraft_versions,
            start_oauth,
            get_accounts,
            set_active_account,
            remove_account,
            create_instance,
            get_instances,
            launch_instance,
            remove_instance,
            open_instance_folder,
            read_instance_data,
            write_instance_data,
        ])
        .setup(|app| {
            let handle: AppHandle = app.handle();
            // refresh ms tokens
            tauri::async_runtime::spawn(async move {
                auth::bearer_token::refresh_bearer_tokens(&handle).await;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
