use std::{env, fs::File, path, io::Read};


pub fn read(path: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let exe_path: path::PathBuf = env::current_exe().unwrap();

    let file_path: path::PathBuf = exe_path
        .parent()
        .unwrap()
        .join(path);

    if !file_path.exists() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "The file does not exist",
        )));
    }

    let mut file: File = File::open(&file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let json_value = serde_json::from_str(&contents)?;

    Ok(json_value)

}
