use std::io::Cursor;

use base64::engine::general_purpose;
use base64::Engine;
use image::imageops::FilterType::Nearest;
use image::{imageops, DynamicImage, ImageBuffer, Rgba};
use reqwest::Client;
use serde_json::Value;
use tauri::Manager;

use crate::common::utils::file::{download_as_json, download_as_vec};
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
    };

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
        let avatar: String = user_avatar(uuid).await;
        let account_info: MinecraftAccount =
            match file::read_as_vec(format!("launcher/auth/{uuid}.json").as_str()).await {
                Ok(account_bytes) => {
                    let mut account_info: MinecraftAccount =
                        serde_json::from_slice(&account_bytes).unwrap();
                    account_info.access_token = String::from(token);
                    account_info.username = String::from(username);
                    account_info.uuid = String::from(uuid);
                    account_info.refresh_token = String::from(refresh_token);
                    account_info.avatar_64px = String::from(avatar);
                    account_info
                }
                Err(_) => MinecraftAccount {
                    username: String::from(username),
                    uuid: String::from(uuid),
                    access_token: String::from(token),
                    refresh_token: String::from(refresh_token),
                    active: true,
                    avatar_64px: avatar,
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

async fn user_avatar(uuid: &str) -> String {
    let response: Value = download_as_json(
        &format!("https://sessionserver.mojang.com/session/minecraft/profile/{uuid}"),
        "",
        &file::ChecksumType::SHA1,
        "",
        false,
        false,
        None,
    )
    .await
    .unwrap_or_default();

    let mut textures: String = String::from("");

    if let Some(properties) = response["properties"].as_array() {
        for property in properties {
            if let Some(name) = property["name"].as_str() {
                if name == "textures" {
                    if let Some(value) = property["value"].as_str() {
                        textures = value.to_string();
                    }
                }
            }
        }
    }
    if !textures.is_empty() {
        let decoded: Vec<u8> = general_purpose::STANDARD.decode(textures).unwrap();
        let json: Value = serde_json::from_slice(&decoded).unwrap();
        if let Some(skin_url) = json["textures"]["SKIN"]["url"].as_str() {
            let skin_bytes: Vec<u8> = download_as_vec(
                skin_url,
                "",
                &file::ChecksumType::SHA1,
                "",
                false,
                false,
                None,
            )
            .await
            .unwrap();

            let mut skin: DynamicImage = image::load_from_memory(&skin_bytes).unwrap();
            let base: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
                imageops::crop(&mut skin, 8, 8, 8, 8).to_image();
            let overlay: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
                imageops::crop(&mut skin, 40, 8, 8, 8).to_image();

            let mut result: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::<Rgba<u8>, _>::new(8, 8);

            imageops::overlay(&mut result, &base, 0, 0);
            imageops::overlay(&mut result, &overlay, 0, 0);
            let result: ImageBuffer<Rgba<u8>, Vec<u8>> =
                imageops::resize(&mut result, 64, 64, Nearest);

            let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
            result
                .write_to(&mut buffer, image::ImageOutputFormat::Png)
                .unwrap();
            let base64_image: String = general_purpose::STANDARD.encode(buffer.get_ref());

            return base64_image;
        };
    };
    return String::from("");
}
