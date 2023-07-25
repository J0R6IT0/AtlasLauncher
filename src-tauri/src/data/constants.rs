// auth
pub static MSAUTH_BASE_URL: &str =
    "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
pub static MSAUTH_LIVE_BASE_URL: &str = "https://login.live.com";
pub static MSAUTH_CLIENT_ID: &str = "00000000402b5328";
pub static MSAUTH_REDIRECT_URI: &str = "https://login.live.com/oauth20_desktop.srf";
pub static MSAUTH_ENCODED_REDIRECT_URI: &str = "https%3A%2F%2Flogin.live.com%2Foauth20_desktop.srf";
pub static MSAUTH_SCOPE: &str = "XboxLive.signin%20offline_access";

// skins
pub static FACE_POSITION: (u32, u32, u32, u32) = (8, 8, 8, 8);
pub static FACE_OVERLAY_POSITION: (u32, u32, u32, u32) = (40, 8, 8, 8);

// metadata
pub static MINECRAFT_VERSION_MANIFEST: &str = "https://raw.githubusercontent.com/J0R6IT0/AtlasLauncherResources/main/meta/minecraft/version_manifest.json";
