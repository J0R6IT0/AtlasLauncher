use serde_json::Value;

use super::{fabric, forge, quilt};

pub async fn get_manifest(
    loader: &str,
    app: &tauri::AppHandle,
    instance_name: &str,
    id: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let manifest: serde_json::Value;
    if loader.starts_with("forge-") {
        manifest = forge::download_manifest(&loader.replace("forge-", ""), app, instance_name)
            .await
            .unwrap();
        Ok(manifest)
    } else if loader.starts_with("fabric-") {
        manifest = fabric::download_manifest(&id, &loader.replace("fabric-", ""))
            .await
            .unwrap();
        Ok(manifest)
    } else {
        manifest = quilt::download_manifest(&id, &loader.replace("quilt-", ""))
            .await
            .unwrap();
        Ok(manifest)
    }
}
