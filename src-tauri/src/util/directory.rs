use std::path::{Path, PathBuf};

use tokio::fs;

use crate::APP_DIRECTORY;

/// Converts a relative path to the executable into an absolute path. If the directory does not exist, a new one is created.
///
/// # Examples
/// ```
/// let user_dir = get_path_or_create("cache/users").await;
/// ```
pub async fn get_path_or_create<P>(path: P) -> PathBuf
where
    P: AsRef<Path>,
{
    let path = APP_DIRECTORY.get().unwrap().join(path);

    fs::create_dir_all(&path)
        .await
        .expect("Failed to create directory");
    path
}
