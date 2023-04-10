use reqwest::Client;
use serde_json::{self, Value};
use tauri::Manager;

use crate::auth::login::LoginEventPayload;
use crate::utils::json_to_file;

pub async fn login(
    token: &str,
    hash: &str,
    app: &tauri::AppHandle,
    refresh_token: &str,
    from_refresh: bool,
) {
    // We are using the same function for Xbox login (site 0) and minecraft XSTS token (site 1)
    let auth_request: String = format!(
        r#"
        {{
            "identityToken": "XBL3.0 x={hash};{token}"
        }}
    "#
    );

    let json: Value = serde_json::from_str(&auth_request).unwrap();

    let client: Client = Client::new();

    let response: Value = client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&json)
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
                message: format!("Fetching Minecraft profile."),
                status: String::from("Loading"),
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
                message: String::from(
                    "This account does not own Minecraft. An official account is required to play.",
                ),
                status: String::from("Error"),
            },
        )
        .unwrap();
    } else {
        let account_info: String = format!(
            r#"
                {{
                    "access_token": "{token}",
                    "username": "{username}",
                    "uuid": "{uuid}",
                    "refresh_token": "{refresh_token}"
                }}
            "#
        );

        let active_account: String = format!(
            r#"
                {{
                    "uuid": "{uuid}"
                }}
            "#
        );
        json_to_file::save(&account_info, &format!("auth/{uuid}.json"));

        if !from_refresh {
            json_to_file::save(&active_account, "auth/active_account.json");
            app.emit_all(
                "auth",
                LoginEventPayload {
                    message: format!("Successfully logged in to account {username}."),
                    status: String::from("Success"),
                },
            )
            .unwrap();
        }
    }
}
