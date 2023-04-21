use crate::common::{
    auth,
    utils::{directory, file},
};

use crate::data::{
    constants,
    models::{LoginEventPayload, MinecraftAccount},
};

use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};
use tauri::{AppHandle, Manager, Window};

pub fn create_login_window(handle: tauri::AppHandle) {
    match handle.get_window("auth") {
        Some(_) => return,
        None => (),
    }

    let url: String = format!("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?client_id={}&response_type=code&response_mode=query&redirect_uri={}&scope={}&prompt=select_account", constants::OAUTH_CLIENT_ID, constants::OAUTH_ENCODED_REDIRECT_URI, constants::OAUTH_SCOPE);

    let app: AppHandle = handle.to_owned();

    tauri::WindowBuilder::new(&handle, "auth", tauri::WindowUrl::App(url.parse().unwrap()))
        .inner_size(500.0, 550.0)
        .title("Sign in to Minecraft")
        .on_navigation(move |url| {
            if url.to_string().starts_with(constants::OAUTH_REDIRECT_URI) {
                if url.to_string().contains("?error") {
                    app.emit_all(
                        "auth",
                        LoginEventPayload {
                            message: format!("Auth canceled."),
                            status: String::from("Hide"),
                        },
                    )
                    .unwrap();
                } else {
                    let code: String = url.to_owned().query_pairs().next().unwrap().1.to_string();

                    let app: AppHandle = app.to_owned();
                    app.emit_all("close-auth-window", ()).unwrap();

                    tauri::async_runtime::spawn(async move {
                        auth::bearer_token::get_bearer_token(code.as_str(), &app, false).await;
                    });
                }
                let window: Window = app.get_window("auth").unwrap();
                window.close().unwrap();
            }
            true
        })
        .build()
        .unwrap()
        .on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                handle
                    .emit_all(
                        "auth",
                        LoginEventPayload {
                            message: format!("Window closed"),
                            status: String::from("Hide"),
                        },
                    )
                    .unwrap();
            }
        });
}

pub fn get_accounts() -> Vec<MinecraftAccount> {
    let mut accounts: Vec<MinecraftAccount> = Vec::new();

    let auth_path: PathBuf = directory::check_directory_sync("launcher/auth");

    for entry in fs::read_dir(auth_path).unwrap() {
        let entry: DirEntry = entry.unwrap();
        let path: PathBuf = entry.path();

        let contents: Vec<u8> = fs::read(&path).unwrap();
        let account: MinecraftAccount = serde_json::from_slice(&contents).unwrap();

        accounts.push(account);
    }

    accounts
}

pub fn remove_account(uuid: &str) {
    file::delete(format!("launcher/auth/{}.json", uuid).as_str());
}

pub fn get_active_account() -> Result<MinecraftAccount, Box<dyn std::error::Error>> {
    let accounts: Vec<MinecraftAccount> = get_accounts();

    for account in accounts {
        if account.active {
            return Ok(account);
        }
    }

    return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "No active account found",
    )));
}

pub fn set_active_account(uuid: &str) {
    let auth_path: PathBuf = directory::check_directory_sync("launcher/auth");

    for entry in fs::read_dir(auth_path).unwrap() {
        let entry: DirEntry = entry.unwrap();
        let path: PathBuf = entry.path();

        let contents: Vec<u8> = fs::read(&path).unwrap();
        let mut account: MinecraftAccount = serde_json::from_slice(&contents).unwrap();
        if account.uuid == uuid {
            account.active = true;
        } else {
            account.active = false;
        }

        file::write_vec(
            &serde_json::to_vec(&account).unwrap(),
            format!("launcher/auth/{}.json", { account.uuid }).as_str(),
        )
        .unwrap();
    }
}
