pub mod auth;
pub mod metadata;
pub mod events;
pub mod instance;

#[derive(Clone, Copy)]
pub enum ChecksumType {
    SHA1,
    SHA256,
    MD5,
}
