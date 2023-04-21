use crate::auth::{login, xbox};
use crate::data::constants;
use crate::data::models::{LoginEventPayload, MinecraftAccount};
use reqwest::Client;
use serde_json::Value;
use tauri::Manager;

pub async fn get_bearer_token(code: &str, app: &tauri::AppHandle, is_refresh: bool) {
    let client: Client = Client::new();

    let response: Value = client
        .post("https://login.live.com/oauth20_token.srf")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Origin", "https://login.live.com")
        .form(&vec![
            ("client_id", constants::OAUTH_CLIENT_ID),
            ("scope", "XboxLive.signin offline_access"),
            (if is_refresh { "refresh_token" } else { "code" }, &code),
            (
                "grant_type",
                if is_refresh {
                    "refresh_token "
                } else {
                    "authorization_code"
                },
            ),
            ("redirect_uri", constants::OAUTH_REDIRECT_URI),
        ])
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let token: &str = response["access_token"].as_str().unwrap_or("");
    let refresh_token: &str = response["refresh_token"].as_str().unwrap_or("");

    if !is_refresh {
        app.emit_all(
            "auth",
            LoginEventPayload {
                message: format!("Logging in to Xbox."),
                status: String::from("Loading"),
            },
        )
        .unwrap();
    }

    xbox::xbox_login(token, app, refresh_token, is_refresh).await;
}

pub async fn refresh_bearer_tokens(app: &tauri::AppHandle) {
    let accounts: Vec<MinecraftAccount> = login::get_accounts();

    for account in accounts.iter() {
        get_bearer_token(&account.refresh_token, app, true).await;
    }
}
