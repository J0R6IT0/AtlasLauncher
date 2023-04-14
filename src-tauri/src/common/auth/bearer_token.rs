use crate::{auth::{login, xbox}, common::utils::directory};
use reqwest::Client;
use serde_json::Value;
use tauri::Manager;

use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

pub async fn get_bearer_token(code: &str, app: &tauri::AppHandle) {
    let client: Client = Client::new();

    let response: Value = client
        .post("https://login.live.com/oauth20_token.srf")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Origin", "https://login.live.com")
        .form(&vec![
            ("client_id", "00000000402b5328"),
            ("scope", "XboxLive.signin offline_access"),
            ("code", &code),
            ("grant_type", "authorization_code"),
            ("redirect_uri", "https://login.live.com/oauth20_desktop.srf"),
        ])
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let token: &str = response["access_token"].as_str().unwrap_or("");
    let refresh_token: &str = response["refresh_token"].as_str().unwrap_or("");

    app.emit_all(
        "auth",
        login::LoginEventPayload {
            message: format!("Logging in to Xbox."),
            status: String::from("Loading"),
        },
    )
    .unwrap();

    xbox::login(token, 0, app, refresh_token, false).await;
}

pub async fn refresh_bearer_tokens(app: &tauri::AppHandle) {
    let mut accounts: Vec<login::AccountInfo> = Vec::new();

    let auth_path: PathBuf = directory::check_directory("launcher/auth").await;

    for entry in fs::read_dir(auth_path).unwrap() {
        let entry: DirEntry = entry.unwrap();
        let path: PathBuf = entry.path();
        let file_name: &str = path.file_name().unwrap().to_str().unwrap();

        if !file_name.starts_with("active_account") {
            let contents: String = fs::read_to_string(&path).unwrap();
            let account: login::AccountInfo = serde_json::from_str(&contents).unwrap();
            accounts.push(account);
        }
    }

    let client: Client = Client::new();

    for account in accounts.iter() {
        let response: Value = client
            .post("https://login.live.com/oauth20_token.srf")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Origin", "https://login.live.com")
            .form(&vec![
                ("client_id", "00000000402b5328"),
                ("scope", "XboxLive.signin offline_access"),
                ("refresh_token", &account.refresh_token),
                ("grant_type", "refresh_token"),
            ])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let token: &str = response["access_token"].as_str().unwrap_or("");
        let refresh_token: &str = response["refresh_token"].as_str().unwrap_or("");

        xbox::login(token, 0, app, refresh_token, true).await;
    }
}
