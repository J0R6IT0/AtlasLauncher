use std::{env, fs, path};

pub fn save(data: &str, path: &str) {
    let exe_path: path::PathBuf = env::current_exe().unwrap();

    let file_path: path::PathBuf = exe_path
        .parent()
        .unwrap()
        .join(String::from("launcher/") + path);

    let file: fs::File = fs::File::create(&file_path)
        .map_err(|e| format!("Failed to create file: {}", e))
        .unwrap();

    let json: serde_json::Value = serde_json::from_str(&data).unwrap();

    serde_json::to_writer(file, &json)
        .map_err(|e| format!("Failed to write to file: {}", e))
        .unwrap();
}
