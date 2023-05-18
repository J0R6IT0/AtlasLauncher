use reqwest::{header::USER_AGENT, Client};
use serde_json::Value;

use crate::data::constants::{MODRINTH_BASE, USER_AGENT as PROJECT_USER_AGENT};

pub async fn fetch_modpacks() -> Result<Value, Box<dyn std::error::Error>> {
    let client: Client = Client::new();

    let modpacks: Value = client
        .get(format!(
            "{MODRINTH_BASE}/search?facets=[[\"project_type:modpack\"]]&limit=50"
        ))
        .header(USER_AGENT, PROJECT_USER_AGENT)
        .send()
        .await?
        .json()
        .await?;

    Ok(modpacks)
}
