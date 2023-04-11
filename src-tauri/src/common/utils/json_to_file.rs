use serde_json::Value;
use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::utils::directory_checker::check_directory_sync;

pub fn save(data: &str, path: &str) {
    let exe_path: PathBuf = env::current_exe().unwrap();

    let file_path: PathBuf = exe_path.parent().unwrap().join(path);
    let path: &Path = file_path.parent().unwrap();

    check_directory_sync(path.to_str().unwrap());

    let file: File = File::create(&file_path)
        .map_err(|e| format!("Failed to create file: {}", e))
        .unwrap();
    let mut writer: BufWriter<File> = BufWriter::new(file);

    let json: Value = serde_json::from_str(&data).unwrap();

    serde_json::to_writer(&mut writer, &json)
        .map_err(|e| format!("Failed to write to file: {}", e))
        .unwrap();

    writer.flush().unwrap();
}
