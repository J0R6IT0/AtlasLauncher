use std::fs::OpenOptions;
use std::io::Write;

use super::directory::check_directory_sync;

pub fn write_line(text: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(check_directory_sync("launcher/logs/atlas.log"))
        .unwrap();

    writeln!(file, "{text}").unwrap();
}
