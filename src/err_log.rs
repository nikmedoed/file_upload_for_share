use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub(crate) fn log_error(error_message: &str) {
    let exe_path = std::env::current_exe().unwrap();
    let log_path = exe_path
        .parent()
        .unwrap_or(Path::new("."))
        .join("err.txt");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", error_message) {
        eprintln!("Failed to write to log file: {}", e);
    }
}
