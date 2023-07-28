use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct BearerTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxAuthRequest {
    pub properties: XboxAuthProperties,
    pub relying_party: String,
    pub token_type: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxAuthProperties {
    pub auth_method: String,
    pub site_name: String,
    pub rps_ticket: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxAuthResponse {
    pub display_claims: XboxAuthResponseDisplayClaims,
    pub token: String,
}

#[derive(Deserialize)]
pub struct XboxAuthResponseDisplayClaims {
    pub xui: Vec<XboxAuthResponseDisplayClaimsXui>,
}

#[derive(Deserialize)]
pub struct XboxAuthResponseDisplayClaimsXui {
    pub uhs: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MinecraftXSTSRequest {
    pub properties: MinecraftXSTSProperties,
    pub relying_party: String,
    pub token_type: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MinecraftXSTSProperties {
    pub sandbox_id: String,
    pub user_tokens: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftLoginRequest {
    pub identity_token: String,
}

#[derive(Deserialize)]
pub struct MinecraftLoginResponse {
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct MinecraftAccountInfoResponse {
    pub id: String,
    pub name: String,
}
