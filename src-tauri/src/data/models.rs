use serde::{Deserialize, Serialize};

// Minecraft

#[derive(Serialize, Deserialize, Clone)]
pub struct MinecraftVersionData {
    pub id: String,
    pub url: String,
    pub r#type: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InstanceInfo {
    pub name: String,
    pub version: String,
    pub background: String,
    pub icon: String,
    pub version_type: String,
    pub height: String,
    pub width: String,
    pub fullscreen: bool,
    pub modloader: String,
}

// Auth

#[derive(Serialize, Deserialize)]
pub struct MinecraftAccount {
    pub username: String,
    pub uuid: String,
    pub refresh_token: String,
    pub access_token: String,
    pub active: bool,
    pub avatar_64px: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxAuthRequest {
    pub properties: XboxAuthProperties,
    pub relying_party: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxAuthProperties {
    pub auth_method: String,
    pub site_name: String,
    pub rps_ticket: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MinecraftXSTSRequest {
    pub properties: MinecraftXSTSProperties,
    pub relying_party: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MinecraftXSTSProperties {
    pub sandbox_id: String,
    pub user_tokens: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftLoginRequest {
    pub identity_token: String,
}

// Events

#[derive(Clone, Serialize)]
pub struct BaseEventPayload {
    pub message: String,
    pub status: String,
}

#[derive(Clone, Serialize)]
pub struct LoginEventPayload {
    pub base: BaseEventPayload,
}

#[derive(Clone, Serialize)]
pub struct StartInstanceEventPayload {
    pub base: BaseEventPayload,
}

#[derive(Clone, Serialize)]
pub struct DownloadInstanceEventPayload {
    pub base: BaseEventPayload,
    pub total: u64,
    pub downloaded: u64,
    pub name: String,
}
