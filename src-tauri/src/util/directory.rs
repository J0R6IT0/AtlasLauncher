use std::{ path::PathBuf, env };

use tokio::fs;

pub async fn check_directory(path: &str) -> PathBuf {
    let path = env
        ::current_exe()
        .expect("Failed to get executable path")
        .parent()
        .unwrap()
        .join(path);

    if path.exists() {
        return path;
    }
    fs::create_dir_all(&path).await.expect("Failed to create directory");
    path
}
