use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum EventStatus {
    Success,
    Loading,
    Error,
}

#[derive(Clone, Serialize)]
pub struct BaseEventPayload {
    pub message: String,
    pub status: EventStatus,
}

#[derive(Clone, Serialize)]
pub struct MSAuthEventPayload {
    pub base: BaseEventPayload,
}
