use serde::{ Deserialize, Serialize };

use super::enums::EventStatus;

// auth
#[derive(Serialize, Deserialize)]
pub struct MinecraftAccount {
    pub username: String,
    pub uuid: String,
    pub refresh_token: String,
    pub access_token: String,
    pub active: bool,
    pub avatar_64px: String,
}

// events
#[derive(Clone, Serialize)]
pub struct BaseEventPayload {
    pub message: String,
    pub status: EventStatus,
}

#[derive(Clone, Serialize)]
pub struct MSAuthEventPayload {
    pub base: BaseEventPayload,
}
