use std::io::Cursor;

use base64::{engine::general_purpose, Engine};
use image::{
    imageops::{self, FilterType::Nearest},
    DynamicImage, ImageBuffer, Rgba,
};
use serde_json::Value;

use crate::{
    data::constants::{FACE_OVERLAY_POSITION, FACE_POSITION},
    models::ChecksumType,
    util::file::download_as_vec,
};

pub async fn user_skin(uuid: &str) -> Result<Vec<u8>, &'static str> {
    let response: Value = match (match reqwest::get(format!(
        "https://sessionserver.mojang.com/session/minecraft/profile/{uuid}"
    ))
    .await
    {
        Ok(response) => response,
        Err(_) => {
            return Err("Error receiving response.");
        }
    })
    .json()
    .await
    {
        Ok(response) => response,
        Err(_) => {
            return Err("Error parsing response json.");
        }
    };

    let mut textures: String = String::from("");

    if let Some(properties) = response["properties"].as_array() {
        for property in properties {
            if let Some(name) = property["name"].as_str() {
                if name == "textures" {
                    if let Some(value) = property["value"].as_str() {
                        textures = value.to_string();
                    }
                }
            }
        }
    }

    if textures.is_empty() {
        return Err("Error fetching user textures.");
    }

    let decoded: Vec<u8> = general_purpose::STANDARD
        .decode(textures)
        .unwrap_or_default();
    let json: Value = serde_json::from_slice(&decoded).unwrap_or_default();

    if let Some(skin_url) = json["textures"]["SKIN"]["url"].as_str() {
        let skin_bytes: Vec<u8> =
            download_as_vec(skin_url, "", ChecksumType::SHA1, "", false).await?;

        return Ok(skin_bytes);
    }

    Err("Error fetching user skin.")
}

pub async fn user_avatar(uuid: &str) -> Result<String, &'static str> {
    let skin_bytes = user_skin(uuid).await?;

    let mut skin = match image::load_from_memory(&skin_bytes) {
        Ok(skin) => skin,
        Err(_) => {
            return Err("Error loading image.");
        }
    };

    let base = crop_image(&mut skin, FACE_POSITION);
    let overlay = crop_image(&mut skin, FACE_OVERLAY_POSITION);

    let mut result = ImageBuffer::<Rgba<u8>, _>::new(8, 8);

    imageops::overlay(&mut result, &base, 0, 0);
    imageops::overlay(&mut result, &overlay, 0, 0);

    let result = imageops::resize(&result, 64, 64, Nearest);

    let mut buffer = Cursor::new(Vec::new());
    match result.write_to(&mut buffer, image::ImageOutputFormat::Png) {
        Ok(_) => {}
        Err(_) => {
            return Err("Error writing to buffer.");
        }
    }

    let base64_image = general_purpose::STANDARD.encode(buffer.get_ref());

    Ok(base64_image)
}

fn crop_image(
    image: &mut DynamicImage,
    position: (u32, u32, u32, u32),
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    imageops::crop(image, position.0, position.1, position.2, position.3).to_image()
}
