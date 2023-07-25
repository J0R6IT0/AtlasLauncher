use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use tauri::{AppHandle, Manager, WindowBuilder, WindowEvent, WindowUrl};

use crate::{
    data::{
        constants::{
            MSAUTH_BASE_URL, MSAUTH_CLIENT_ID, MSAUTH_ENCODED_REDIRECT_URI, MSAUTH_LIVE_BASE_URL,
            MSAUTH_REDIRECT_URI, MSAUTH_SCOPE,
        },
        enums::EventStatus,
        models::{BaseEventPayload, MSAuthEventPayload, MinecraftAccount},
    },
    minecraft::skin::user_avatar,
    util::file::{self, read_as_value, write_value},
    APP,
};

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxAuthRequest {
    pub properties: XboxAuthProperties,
    pub relying_party: String,
    pub token_type: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxAuthProperties {
    pub auth_method: String,
    pub site_name: String,
    pub rps_ticket: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MinecraftXSTSRequest {
    pub properties: MinecraftXSTSProperties,
    pub relying_party: String,
    pub token_type: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MinecraftXSTSProperties {
    pub sandbox_id: String,
    pub user_tokens: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftLoginRequest {
    pub identity_token: String,
}

#[tauri::command]
pub async fn start_msauth(app_handle: AppHandle) {
    // check if the msauth window already exists (auth already in progress)
    if app_handle.get_window("msauth").is_some() {
        return;
    }

    // create a new window for the ms authentication
    WindowBuilder::new(
        &app_handle,
        "msauth",
        WindowUrl::External(build_url().parse().unwrap()),
    )
    .inner_size(500.0, 550.0)
    .title("Sign in to Minecraft")
    .center()
    .on_navigation(move |url| {
        let url_string = url.to_string();
        if url_string.starts_with(MSAUTH_REDIRECT_URI) {
            if url_string.contains("?error") {
                emit_event(EventStatus::Error, "Error logging in");
            } else {
                // obtain the code from the url and proceed with the authentication
                let code = url.query_pairs().next().unwrap().1.to_string();

                tauri::async_runtime::spawn(async move {
                    match auth_flow(&code, false).await {
                        Ok(_) => {}
                        Err(err) => emit_event(EventStatus::Error, err),
                    };
                });
            }
            let window = APP.get().unwrap().get_window("msauth").unwrap();
            window.close().unwrap();
        }
        true
    })
    .build()
    .unwrap()
    .on_window_event(move |event| {
        // check if the user closes the auth window
        if let WindowEvent::CloseRequested { .. } = event {
            emit_event(EventStatus::Error, "Window closed");
        }
    });
}

async fn auth_flow(code: &str, is_refresh: bool) -> Result<(), &'static str> {
    let client = Client::new();

    emit_event(EventStatus::Loading, "Obtaining bearer token.");
    let (bearer_token, refresh_token) = bearer_token(&client, code, is_refresh).await?;

    emit_event(EventStatus::Loading, "Logging in to Xbox.");
    let xbox_token = xbox_login(&client, &bearer_token).await?;
    let (xsts_token, uhs) = xsts_token(&client, &xbox_token).await?;

    emit_event(EventStatus::Loading, "Logging in to Minecraft.");
    let access_token = minecraft_login(&client, &xsts_token, &uhs).await?;

    emit_event(EventStatus::Loading, "Fetching Minecraft profile.");
    let (uuid, username) = account_info(&client, &access_token).await?;

    emit_event(EventStatus::Loading, "Fetching user skin.");
    let avatar = user_avatar(&uuid).await?;

    let mut accounts: Vec<MinecraftAccount> =
        match file::read_as_value("launcher/accounts.json").await {
            Ok(accounts) => accounts,
            Err(_) => vec![],
        };

    if let Some(account) = accounts.iter_mut().find(|account| account.uuid == uuid) {
        // Update the existing account without changing the active status
        account.access_token = access_token.clone();
        account.avatar_64px = avatar.clone();
        account.username = username.clone();
        account.refresh_token = refresh_token.clone();
    } else {
        // Create a new account changing the active status
        let new_account = MinecraftAccount {
            access_token: access_token.clone(),
            avatar_64px: avatar.clone(),
            username: username.clone(),
            uuid: uuid.clone(),
            refresh_token: refresh_token.clone(),
            active: true,
        };

        accounts.push(new_account);
    }

    file::write_value(&accounts, "launcher/accounts.json").await?;

    set_active_account(&uuid).await?;

    emit_event(EventStatus::Success, "Success");

    Ok(())
}

async fn bearer_token(
    client: &Client,
    code: &str,
    is_refresh: bool,
) -> Result<(String, String), &'static str> {
    let response: Value = match (match client
        .post(format!("{}/oauth20_token.srf", MSAUTH_LIVE_BASE_URL))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Origin", MSAUTH_LIVE_BASE_URL)
        .form(&vec![
            ("client_id", MSAUTH_CLIENT_ID),
            (if is_refresh { "refresh_token" } else { "code" }, code),
            (
                "grant_type",
                if is_refresh {
                    "refresh_token "
                } else {
                    "authorization_code"
                },
            ),
            ("redirect_uri", MSAUTH_REDIRECT_URI),
        ])
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return Err("Error receiving response.");
        }
    })
    .json()
    .await
    {
        Ok(response) => response,
        Err(_) => {
            return Err("Error parsing response json.");
        }
    };

    let token = response["access_token"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let refresh_token = response["refresh_token"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    if token.is_empty() || refresh_token.is_empty() {
        return Err("Error obtaining bearer token.");
    }

    Ok((token, refresh_token))
}

async fn xbox_login(client: &Client, bearer_token: &str) -> Result<String, &'static str> {
    let properties: XboxAuthProperties = XboxAuthProperties {
        auth_method: String::from("RPS"),
        site_name: String::from("user.auth.xboxlive.com"),
        rps_ticket: format!("d={bearer_token}"),
    };
    let xbox_request: XboxAuthRequest = XboxAuthRequest {
        properties,
        relying_party: String::from("http://auth.xboxlive.com"),
        token_type: String::from("JWT"),
    };

    let response: Value = match (match client
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&xbox_request)
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return Err("Error receiving response.");
        }
    })
    .json()
    .await
    {
        Ok(response) => response,
        Err(_) => {
            return Err("Error parsing response json.");
        }
    };

    let token = response["Token"].as_str().unwrap_or_default().to_string();
    if token.is_empty() {
        return Err("Error logging in to Xbox.");
    }

    Ok(token)
}

async fn xsts_token(client: &Client, xbox_token: &str) -> Result<(String, String), &'static str> {
    let properties: MinecraftXSTSProperties = MinecraftXSTSProperties {
        sandbox_id: String::from("RETAIL"),
        user_tokens: vec![format!("{}", xbox_token)],
    };
    let minecraft_request: MinecraftXSTSRequest = MinecraftXSTSRequest {
        properties,
        relying_party: String::from("rp://api.minecraftservices.com/"),
        token_type: String::from("JWT"),
    };

    let response: Value = match (match client
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&minecraft_request)
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return Err("Error receiving response.");
        }
    })
    .json()
    .await
    {
        Ok(response) => response,
        Err(_) => {
            return Err("Error parsing response json.");
        }
    };

    let token = response["Token"].as_str().unwrap_or_default().to_string();
    let uhs = response["DisplayClaims"]["xui"][0]["uhs"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    if token.is_empty() || uhs.is_empty() {
        return Err("Error obtaining XSTS token.");
    }

    Ok((token, uhs))
}

async fn minecraft_login(
    client: &Client,
    xsts_token: &str,
    uhs: &str,
) -> Result<String, &'static str> {
    let auth_request: MinecraftLoginRequest = MinecraftLoginRequest {
        identity_token: format!("XBL3.0 x={uhs};{xsts_token}"),
    };

    let response: Value = match (match client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&auth_request)
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return Err("Error receiving response.");
        }
    })
    .json()
    .await
    {
        Ok(response) => response,
        Err(_) => {
            return Err("Error parsing response json.");
        }
    };

    let token = response["access_token"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    if token.is_empty() {
        return Err("Error obtaining Minecraft token.");
    }

    Ok(token)
}

async fn account_info(
    client: &Client,
    access_token: &str,
) -> Result<(String, String), &'static str> {
    let response: Value = match (match client
        .get("https://api.minecraftservices.com/minecraft/profile")
        .header("Authorization", format!("Bearer {access_token}"))
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return Err("Error receiving response.");
        }
    })
    .json()
    .await
    {
        Ok(response) => response,
        Err(_) => {
            return Err("Error parsing response json.");
        }
    };

    let error = response["error"].as_str().unwrap_or_default().to_string();

    let uuid = response["id"].as_str().unwrap_or_default().to_string();
    let username = response["name"].as_str().unwrap_or_default().to_string();

    if uuid.is_empty() || username.is_empty() || !error.is_empty() {
        return Err(
            "This account does not own Minecraft. An official account is required to play.",
        );
    }

    Ok((uuid, username))
}

fn emit_event<S: AsRef<str>>(status: EventStatus, message: S) {
    APP.get()
        .unwrap()
        .emit_all(
            "msauth",
            MSAuthEventPayload {
                base: BaseEventPayload {
                    message: message.as_ref().to_string(),
                    status,
                },
            },
        )
        .unwrap();
}

fn build_url() -> String {
    format!(
        "{}?client_id={}&response_type=code&response_mode=query&redirect_uri={}&scope={}&prompt=select_account",
        MSAUTH_BASE_URL,
        MSAUTH_CLIENT_ID,
        MSAUTH_ENCODED_REDIRECT_URI,
        MSAUTH_SCOPE
    )
}

#[tauri::command]
pub async fn get_accounts() -> Vec<MinecraftAccount> {
    let mut accounts: Vec<MinecraftAccount> = read_as_value("launcher/accounts.json")
        .await
        .unwrap_or_default();

    // return the accounts in alphabetical order
    accounts.sort_by(|a, b| a.username.cmp(&b.username));

    accounts
}

#[tauri::command]
pub async fn set_active_account(uuid: &str) -> Result<(), &'static str> {
    let mut accounts = get_accounts().await;

    for account in &mut accounts {
        account.active = account.uuid == uuid;
    }

    write_value(&accounts, "launcher/accounts.json").await?;

    Ok(())
}

#[tauri::command]
pub async fn remove_account(uuid: &str) -> Result<(), &'static str> {
    let mut accounts: Vec<MinecraftAccount> = read_as_value("launcher/accounts.json")
        .await
        .unwrap_or_default();

    accounts.retain(|acc| acc.uuid != uuid);

    write_value(&accounts, "launcher/accounts.json").await?;

    Ok(())
}

pub async fn refresh_bearer_tokens() -> Result<(), &'static str> {
    let accounts: Vec<MinecraftAccount> = read_as_value("launcher/accounts.json")
        .await
        .unwrap_or_default();

    for account in accounts.iter() {
        auth_flow(&account.refresh_token, true).await?;
    }

    Ok(())
}
