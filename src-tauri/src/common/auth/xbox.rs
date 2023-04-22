use crate::auth::minecraft;
use crate::data::models::{
    BaseEventPayload, MinecraftXSTSProperties, MinecraftXSTSRequest, XboxAuthProperties,
    XboxAuthRequest, LoginEventPayload,
};
use reqwest::Client;
use serde_json::Value;
use tauri::Manager;

pub async fn xbox_login(
    token: &str,
    app: &tauri::AppHandle,
    refresh_token: &str,
    is_refresh: bool,
) {
    let properties: XboxAuthProperties = XboxAuthProperties {
        auth_method: String::from("RPS"),
        site_name: String::from("user.auth.xboxlive.com"),
        rps_ticket: format!("d={token}"),
    };
    let xbox_request: XboxAuthRequest = XboxAuthRequest {
        properties,
        relying_party: String::from("http://auth.xboxlive.com"),
        token_type: String::from("JWT"),
    };

    let client: Client = Client::new();

    let response: Result<Value, reqwest::Error> = client
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&xbox_request)
        .send()
        .await
        .unwrap()
        .json()
        .await;

    let response: Value = match response {
        Ok(reponse) => reponse,
        Err(_) => return,
    };

    let token: &str = response["Token"].as_str().unwrap_or_default();

    xsts_token(token, app, refresh_token, is_refresh).await
}

async fn xsts_token(token: &str, app: &tauri::AppHandle, refresh_token: &str, is_refresh: bool) {
    let properties: MinecraftXSTSProperties = MinecraftXSTSProperties {
        sandbox_id: String::from("RETAIL"),
        user_tokens: vec![format!("{}", token)],
    };
    let minecraft_request: MinecraftXSTSRequest = MinecraftXSTSRequest {
        properties,
        relying_party: String::from("rp://api.minecraftservices.com/"),
        token_type: String::from("JWT"),
    };

    let client: Client = Client::new();

    let response: Result<Value, reqwest::Error> = client
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&minecraft_request)
        .send()
        .await
        .unwrap()
        .json()
        .await;

    let response = match response {
        Ok(reponse) => reponse,
        Err(_) => return,
    };

    let token: &str = response["Token"].as_str().unwrap_or_default();
    let uhs: &str = response["DisplayClaims"]["xui"][0]["uhs"]
        .as_str()
        .unwrap_or_default();
    if !is_refresh {
        app.emit_all(
            "auth",
            LoginEventPayload {
                base: BaseEventPayload {
                    message: format!("Obtaining Minecraft token."),
                    status: String::from("Loading"),
                },
            },
        )
        .unwrap();
    }
    minecraft::login(token, uhs, app, refresh_token, is_refresh).await;
}
