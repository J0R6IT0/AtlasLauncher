use std::{env, path::PathBuf};
use serde_json::Value;

pub fn read(path: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let exe_path: PathBuf = env::current_exe().unwrap();

    let file_path: PathBuf = exe_path.parent().unwrap().join(path);

    if !file_path.exists() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "The file does not exist",
        )));
    }

    let contents: String = std::fs::read_to_string(&file_path)?;
    let json_value: Value = serde_json::from_str(&contents)?;

    Ok(json_value)
}
