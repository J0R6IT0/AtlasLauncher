use crate::utils::directory_checker::check_directory;
use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

pub async fn get_java_path(version: u8) -> String {
    let path: PathBuf = check_directory(&format!("java/{version}")).await;

    for entry in fs::read_dir(path).unwrap() {
        let entry: DirEntry = entry.unwrap();
        let path: PathBuf = entry.path().join("bin/java.exe");

        return String::from(path.to_str().unwrap());
    }
    return String::from("");
}
