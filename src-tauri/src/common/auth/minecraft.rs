use reqwest::Client;
use serde_json::Value;
use tauri::Manager;

use crate::data::models::{
    BaseEventPayload, LoginEventPayload, MinecraftAccount, MinecraftLoginRequest,
};
use crate::utils::file;

pub async fn login(
    token: &str,
    hash: &str,
    app: &tauri::AppHandle,
    refresh_token: &str,
    from_refresh: bool,
) {
    let auth_request: MinecraftLoginRequest = MinecraftLoginRequest {
        identity_token: format!("XBL3.0 x={hash};{token}"),
    };

    let client: Client = Client::new();

    let response: Value = client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&auth_request)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let token: &str = response["access_token"].as_str().unwrap_or_default();
    if !from_refresh {
        app.emit_all(
            "auth",
            LoginEventPayload {
                base: BaseEventPayload {
                    message: format!("Fetching Minecraft profile."),
                    status: String::from("Loading"),
                },
            },
        )
        .unwrap();
    }

    get_account_info(token, app, refresh_token, from_refresh).await;
}

pub async fn get_account_info(
    token: &str,
    app: &tauri::AppHandle,
    refresh_token: &str,
    from_refresh: bool,
) {
    let client: Client = Client::new();

    let response: Value = client
        .get("https://api.minecraftservices.com/minecraft/profile")
        .header("Authorization", "Bearer ".to_owned() + token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let error: &str = response["error"].as_str().unwrap_or_default();

    let uuid: &str = response["id"].as_str().unwrap_or_default();
    let username: &str = response["name"].as_str().unwrap_or_default();

    if error.len() > 0 {
        app.emit_all(
            "auth",
            LoginEventPayload {
                base: BaseEventPayload {
                    message: String::from(
                        "This account does not own Minecraft. An official account is required to play.",
                    ),
                    status: String::from("Error"),
                },
            },
        )
        .unwrap();
    } else {
        let account_info: MinecraftAccount =
            match file::read_as_vec(format!("launcher/auth/{uuid}.json").as_str()).await {
                Ok(account_bytes) => {
                    let mut account_info: MinecraftAccount =
                        serde_json::from_slice(&account_bytes).unwrap();
                    account_info.access_token = String::from(token);
                    account_info.username = String::from(username);
                    account_info.uuid = String::from(uuid);
                    account_info.refresh_token = String::from(refresh_token);
                    account_info
                }
                Err(_) => MinecraftAccount {
                    username: String::from(username),
                    uuid: String::from(uuid),
                    access_token: String::from(token),
                    refresh_token: String::from(refresh_token),
                    active: true,
                },
            };

        file::write_vec(
            &serde_json::to_vec(&account_info).unwrap(),
            format!("launcher/auth/{}.json", { uuid }).as_str(),
        )
        .unwrap();

        if !from_refresh {
            app.emit_all(
                "auth",
                LoginEventPayload {
                    base: BaseEventPayload {
                        message: format!("Successfully logged in to account {username}."),
                        status: String::from("Success"),
                    },
                },
            )
            .unwrap();
        }
    }
}
