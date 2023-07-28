use std::path::PathBuf;

use tokio::fs;

use crate::APP_DIRECTORY;

pub async fn check_directory<S: AsRef<str>>(path: S) -> PathBuf {
    let path = APP_DIRECTORY.get().unwrap().join(path.as_ref());

    if path.exists() {
        return path;
    }
    fs::create_dir_all(&path)
        .await
        .expect("Failed to create directory");
    path
}
