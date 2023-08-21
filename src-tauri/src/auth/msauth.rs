use reqwest::Client;
use tauri::{AppHandle, Manager, State, WindowBuilder, WindowEvent, WindowUrl};

use crate::{
    data::constants::{
        MSAUTH_BASE_URL, MSAUTH_CLIENT_ID, MSAUTH_ENCODED_REDIRECT_URI, MSAUTH_LIVE_BASE_URL,
        MSAUTH_REDIRECT_URI, MSAUTH_SCOPE,
    },
    managers::account::{AccountManager, MinecraftAccount},
    minecraft::skin::user_avatar,
    models::{
        auth::{
            BearerTokenResponse, MinecraftAccountInfoResponse, MinecraftLoginRequest,
            MinecraftLoginResponse, MinecraftXSTSProperties, MinecraftXSTSRequest,
            XboxAuthProperties, XboxAuthRequest, XboxAuthResponse,
        },
        events::{BaseEventPayload, EventStatus, MSAuthEventPayload},
    },
    APP,
};

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

pub async fn auth_flow(code: &str, is_refresh: bool) -> Result<(), &'static str> {
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

    let account_manager = APP.get().unwrap().state::<AccountManager>();

    let account = MinecraftAccount {
        active: false,
        username,
        uuid,
        refresh_token,
        access_token,
        avatar_64: avatar,
    };

    account_manager.add_account(account, !is_refresh).await?;

    emit_event(EventStatus::Success, "Success");

    Ok(())
}

async fn bearer_token(
    client: &Client,
    code: &str,
    is_refresh: bool,
) -> Result<(String, String), &'static str> {
    let response: BearerTokenResponse = match (match client
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

    Ok((response.access_token, response.refresh_token))
}

async fn xbox_login(client: &Client, bearer_token: &str) -> Result<String, &'static str> {
    let properties = XboxAuthProperties {
        auth_method: String::from("RPS"),
        site_name: String::from("user.auth.xboxlive.com"),
        rps_ticket: format!("d={bearer_token}"),
    };
    let xbox_request = XboxAuthRequest {
        properties,
        relying_party: String::from("http://auth.xboxlive.com"),
        token_type: String::from("JWT"),
    };

    let response: XboxAuthResponse = match (match client
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

    Ok(response.token)
}

async fn xsts_token(client: &Client, xbox_token: &str) -> Result<(String, String), &'static str> {
    let properties = MinecraftXSTSProperties {
        sandbox_id: String::from("RETAIL"),
        user_tokens: vec![format!("{}", xbox_token)],
    };
    let minecraft_request = MinecraftXSTSRequest {
        properties,
        relying_party: String::from("rp://api.minecraftservices.com/"),
        token_type: String::from("JWT"),
    };

    let response: XboxAuthResponse = match (match client
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

    Ok((response.token, response.display_claims.xui[0].uhs.clone()))
}

async fn minecraft_login(
    client: &Client,
    xsts_token: &str,
    uhs: &str,
) -> Result<String, &'static str> {
    let auth_request = MinecraftLoginRequest {
        identity_token: format!("XBL3.0 x={uhs};{xsts_token}"),
    };

    let response: MinecraftLoginResponse = match (match client
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

    Ok(response.access_token)
}

async fn account_info(
    client: &Client,
    access_token: &str,
) -> Result<(String, String), &'static str> {
    let response: MinecraftAccountInfoResponse = match (match client
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

    Ok((response.id, response.name))
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
pub async fn get_accounts(
    account_manager: State<'_, AccountManager>,
) -> Result<Vec<MinecraftAccount>, &'static str> {
    let mut accounts = account_manager.accounts();
    accounts.sort_unstable();
    Ok(accounts)
}

#[tauri::command]
pub async fn set_active_account(
    account_manager: State<'_, AccountManager>,
    uuid: &str,
) -> Result<(), &'static str> {
    account_manager.set_active_account(uuid).await?;
    Ok(())
}

#[tauri::command]
pub async fn remove_account(
    account_manager: State<'_, AccountManager>,
    uuid: &str,
) -> Result<(), &'static str> {
    account_manager.remove_account(uuid).await?;
    Ok(())
}
