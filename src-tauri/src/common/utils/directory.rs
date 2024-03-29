use std::{env, fs, path::PathBuf};

pub async fn check_directory(path: &str) -> PathBuf {
    check_directory_sync(path)
}

pub fn check_directory_sync(path: &str) -> PathBuf {
    let path: PathBuf = env::current_exe()
        .expect("Failed to get executable path")
        .parent()
        .unwrap()
        .join(path);

    if path.exists() {
        return path;
    }
    fs::create_dir_all(&path).expect("Failed to create directory");
    path
}
