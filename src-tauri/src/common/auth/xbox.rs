use crate::auth::{login, minecraft};
use async_recursion::async_recursion;
use reqwest::Client;
use serde_json::Value;
use tauri::Manager;

#[async_recursion]
pub async fn login(
    token: &str,
    site: u8,
    app: &tauri::AppHandle,
    refresh_token: &str,
    from_refresh: bool,
) {
    // we use the same function for xbox login (site 0) and minecraft xsts token (site 1)
    let auth_request: String = if site == 0 {
        format!(
            r#"
            {{
                "Properties": {{
                    "AuthMethod": "RPS",
                    "SiteName": "user.auth.xboxlive.com",
                    "RpsTicket": "d={token}"
                }},
                "RelyingParty": "http://auth.xboxlive.com",
                "TokenType": "JWT"
            }}
            "#
        )
    } else {
        format!(
            r#"
            {{
                "Properties": {{
                    "SandboxId": "RETAIL",
                    "UserTokens": [
                        "{token}"
                    ]
                }},
                "RelyingParty": "rp://api.minecraftservices.com/",
                "TokenType": "JWT"
             }}
            "#
        )
    };

    let json: Value = serde_json::from_str(&auth_request).unwrap();

    let client: Client = Client::new();

    let response: Result<Value, reqwest::Error> = client
        .post(if site == 0 {
            "https://user.auth.xboxlive.com/user/authenticate"
        } else {
            "https://xsts.auth.xboxlive.com/xsts/authorize"
        })
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&json)
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
    if site == 0 {
        login(token, 1, app, refresh_token, from_refresh).await;
    } else {
        if !from_refresh {
            app.emit_all(
                "auth",
                login::LoginEventPayload {
                    message: format!("Obtaining Minecraft token."),
                    status: String::from("Loading"),
                },
            )
            .unwrap();
        }
        minecraft::login(token, uhs, app, refresh_token, from_refresh).await;
    }
}
