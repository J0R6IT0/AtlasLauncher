use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default, Clone, Serialize)]
pub struct MinecraftVersionManifest {
    pub versions: Vec<MinecraftVersion>,
    pub data: HashMap<String, String>,
}

#[derive(Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftVersion {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    pub release_time: String,
    pub sha1: String,
}
