use std::fs::OpenOptions;
use std::io::Write;

pub async fn log_to_file(log_message: &str) {
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("debug.txt")
        .unwrap();
    writeln!(log_file, "{}", log_message).unwrap();
}
