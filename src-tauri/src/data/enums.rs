use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum EventStatus {
    Success,
    Loading,
    Error,
}

#[derive(Clone, Copy)]
pub enum ChecksumType {
    SHA1,
    SHA256,
    MD5,
}
