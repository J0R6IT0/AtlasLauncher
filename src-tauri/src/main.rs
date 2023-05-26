// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::utils::log::write_line;
use serde_json::Value;
use std::env;
use tauri::AppHandle;

mod common;
mod data;
use common::{
    auth, java,
    minecraft::{
        self,
        versions::{get_fabric_loader_versions, get_fabric_mc_versions},
    },
    modpacks::modrinth::fetch_modpacks,
    utils,
};
use data::models::{self, InstanceInfo};

#[tauri::command]
async fn get_minecraft_versions() -> Result<Vec<models::MinecraftVersionData>, ()> {
    match minecraft::versions::get_versions().await {
        Ok(version_list) => Ok(version_list),
        Err(_) => Ok([].to_vec()),
    }
}

#[tauri::command]
async fn get_forge_versions() -> Result<Value, ()> {
    match minecraft::versions::get_forge_versions().await {
        Ok(version_list) => Ok(version_list),
        Err(_) => Ok(serde_json::from_str("{}").unwrap()),
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
async fn create_instance(
    name: &str,
    id: &str,
    modloader: &str,
    handle: tauri::AppHandle,
) -> Result<(), ()> {
    minecraft::instance::create_instance(id, name, modloader, &handle).await;
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
async fn write_instance_data(
    name: &str,
    data: InstanceInfo,
    handle: tauri::AppHandle,
) -> Result<(), ()> {
    minecraft::instance::write_instance(name, data, &handle).await;
    Ok(())
}

// fabric and quilt
#[tauri::command]
async fn get_fabric_minecraft_versions(is_quilt: bool) -> Vec<Value> {
    match get_fabric_mc_versions(is_quilt).await {
        Ok(versions) => versions,
        Err(_) => [].to_vec(),
    }
}

#[tauri::command]
async fn get_fabric_versions(is_quilt: bool) -> Vec<Value> {
    match get_fabric_loader_versions(is_quilt).await {
        Ok(versions) => versions,
        Err(_) => [].to_vec(),
    }
}

#[tauri::command]
async fn get_modrinth_modpacks() -> Value {
    match fetch_modpacks().await {
        Ok(value) => value,
        Err(_) => serde_json::from_str("[\"hits\": []]").unwrap(),
    }
}

#[tokio::main]
async fn main() {
    // to avoid problems due to having multiple async runtimes running
    tauri::async_runtime::set(tokio::runtime::Handle::current());

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
            write_instance_data,
            get_forge_versions,
            get_fabric_minecraft_versions,
            get_fabric_versions,
            get_modrinth_modpacks,
        ])
        .setup(|app| {
            let handle: AppHandle = app.handle();
            // refresh ms tokens
            tauri::async_runtime::spawn(async move {
                auth::bearer_token::refresh_bearer_tokens(&handle).await;
            });

            // update the version manifest
            tauri::async_runtime::spawn(async move {
                match minecraft::versions::download_version_manifests().await {
                    Ok(_) => println!("Version manifest successfully updated"),
                    Err(err) => {
                        write_line(&format!("Error updating manifest: {:?}", err));
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
