use std::sync::RwLock;

use crate::{
    data::constants::MINECRAFT_VERSION_MANIFEST,
    models::{
        metadata::{MinecraftVersion, MinecraftVersionManifest},
        ChecksumType,
    },
    util::file::{download_as_json, read_as_value},
};

pub struct MetadataManager {
    minecraft_manifest: RwLock<MinecraftVersionManifest>,
}

impl MetadataManager {
    pub async fn init() -> Self {
        let minecraft_manifest: MinecraftVersionManifest =
            read_as_value("launcher/meta/minecraft/version_manifest.json")
                .await
                .unwrap_or_default();

        Self {
            minecraft_manifest: RwLock::new(minecraft_manifest),
        }
    }

    /// Refreshes every version manifest
    pub async fn refesh_manifests(&self) -> Result<(), &'static str> {
        self.refresh_minecraft_manifest().await?;

        Ok(())
    }

    /// Get the Minecraft versions
    pub async fn get_minecraft_versions(&self) -> Result<Vec<MinecraftVersion>, &'static str> {
        match self.minecraft_manifest.read() {
            Ok(read_lock) => Ok(read_lock.versions.clone()),
            Err(_) => Err("Error updating version manifest"),
        }
    }

    /// Refreshes the Minecraft version manifest
    async fn refresh_minecraft_manifest(&self) -> Result<(), &'static str> {
        let manifest: MinecraftVersionManifest = download_as_json(
            MINECRAFT_VERSION_MANIFEST,
            "",
            ChecksumType::SHA1,
            "launcher/meta/minecraft/version_manifest.json",
            true,
        )
        .await?;

        match self.minecraft_manifest.write() {
            Ok(mut write_lock) => *write_lock = manifest,
            Err(_) => return Err("Error updating version manifest"),
        }

        Ok(())
    }
}
