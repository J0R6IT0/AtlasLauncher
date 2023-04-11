use crate::common::auth;
use crate::common::utils;
use crate::common::utils::file_to_json;

use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    env,
    fs::{self, DirEntry},
    path::PathBuf,
    thread,
    time::Duration,
};
use tauri::Manager;

#[derive(Clone, Serialize)]
pub struct LoginEventPayload {
    pub message: String,
    pub status: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub username: String,
    pub uuid: String,
    pub refresh_token: String,
}

pub fn create_login_window(handle: tauri::AppHandle) {
    match handle.get_window("auth") {
        Some(window) => window.close(),
        None => Ok(()),
    }
    .unwrap();

    let url: String = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize".to_owned()
        // This is the official launcher ID
        + "?client_id=00000000402b5328"
        + "&response_type=code"
        + "&response_mode=query"
        + "&redirect_uri=https%3A%2F%2Flogin.live.com%2Foauth20_desktop.srf"
        + "&scope=XboxLive.signin%20offline_access"
        + "&prompt=select_account";

    let auth_window: tauri::Window = tauri::WindowBuilder::new(
        &handle,
        "auth",
        tauri::WindowUrl::External(url.parse().unwrap()),
    )
    .inner_size(500.0, 550.0)
    .title("Sign in to Minecraft")
    .build()
    .unwrap();

    let app = handle.clone();

    tauri::async_runtime::spawn(async move {
        auth::server::start_server(&handle).await;
    });

    thread::spawn(move || loop {
        if let Err(_) = auth_window.is_visible() {
            app.emit_all(
                "auth",
                LoginEventPayload {
                    message: format!("Window closed"),
                    status: String::from("Hide"),
                },
            )
            .unwrap();
            if let Err(err) = auth_window.close() {
                println!("Error closing window: {:?}", err);
            }
            return;
        }

        let js_code: &str = r#"
                    let currentLocation = window.location.href;
                    if (currentLocation.startsWith('https://login.live.com/oauth20_desktop.srf?code=')) {
                        currentLocation = currentLocation.replaceAll(':', '').replaceAll('/', '');
                        window.location.replace('http://localhost:7222/' + currentLocation);
                    }
                "#;
        match auth_window.eval(js_code) {
            Ok(_) => (),
            Err(_) => continue,
        }

        thread::sleep(Duration::from_secs(1));
    });
}

pub fn get_accounts() -> Vec<AccountInfo> {
    let mut accounts: Vec<AccountInfo> = Vec::new();

    let auth_path: PathBuf = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("launcher/auth");

    for entry in fs::read_dir(auth_path).unwrap() {
        let entry: DirEntry = entry.unwrap();
        let path: PathBuf = entry.path();
        let file_name: &str = path.file_name().unwrap().to_str().unwrap();

        if !file_name.starts_with("active_account") {
            let contents: String = fs::read_to_string(&path).unwrap();
            let account: AccountInfo = serde_json::from_str(&contents).unwrap();
            accounts.push(account);
        }
    }

    accounts
}

pub fn get_active_account() -> String {
    let path: PathBuf = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("launcher/auth/active_account.json");

    if !path.exists() {
        return String::from("");
    }

    let content = fs::read_to_string(&path).unwrap();

    let my_json: serde_json::Value = serde_json::from_str(&content).unwrap();

    my_json["uuid"].as_str().unwrap().to_string()
}

pub async fn get_active_account_info() -> serde_json::Value {
    let active_account = get_active_account();

    let account = file_to_json::read(format!("launcher/auth/{active_account}.json").as_str()).unwrap();

    account
}

pub fn set_active_account(uuid: &str) {
    let active_account: String = format!(
        r#"
            {{
                "uuid": "{uuid}"
            }}
        "#
    );

    utils::json_to_file::save(&active_account, "launcher/auth/active_account.json");
}

pub fn remove_account(uuid: &str) {
    let path: PathBuf = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("launcher/auth/{}.json", uuid));

    if !path.exists() {
        return;
    }

    fs::remove_file(path).unwrap();

    let active_account: String = get_active_account();

    if String::from(uuid) == active_account {
        let path: PathBuf = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("launcher/auth/active_account.json");

        fs::remove_file(path).unwrap();
    }
}
