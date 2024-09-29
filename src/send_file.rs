use crate::err_log::log_error;
use ssh2::Session;
use std::fs::File;
use std::net::TcpStream;
use std::path::Path;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use chrono::Utc;
use notify_rust::Notification;
use std::io::{self};

struct Settings {
    server: String,
    username: String,
    password: String,
    link_template: String,
    remote_path: String,
}

impl Settings {
    pub(crate) fn new() -> Self {
        Settings {
            server: env!("SERVER").to_string(),
            username: env!("USERNAME").to_string(),
            password: env!("PASSWORD").to_string(),
            link_template: env!("LINK_TEMPLATE").to_string(),
            remote_path: env!("REMOTE_PATH").to_string(),
        }
    }
}

fn send_file(
    file_path: &str,
    remote_filename: &str,
    settings: &Settings,
) -> Result<(), Box<dyn std::error::Error>> {
    let tcp = TcpStream::connect(format!("{}:22", settings.server)).map_err(|e| {
        log_error(&format!("Error connecting to server: {}", e));
        e
    })?;

    let mut session = Session::new().map_err(|e| {
        log_error(&format!("Error creating session: {}", e));
        e
    })?;

    session.set_tcp_stream(tcp);
    session.handshake().map_err(|e| {
        log_error(&format!("Error during SSH handshake: {}", e));
        e
    })?;

    session.userauth_password(&settings.username, &settings.password).map_err(|e| {
        log_error(&format!("Error during authentication: {}", e));
        e
    })?;

    if !session.authenticated() {
        log_error("Authentication error");
        return Err("Authentication error".into());
    }

    let remote_file_path = format!("{}/{}", settings.remote_path, remote_filename);
    let mut remote_file = session.scp_send(
        Path::new(&remote_file_path),
        0o644,
        std::fs::metadata(file_path)?.len(),
        None,
    ).map_err(|e| {
        log_error(&format!("Error opening remote file: {}", e));
        e
    })?;

    let mut local_file = File::open(file_path).map_err(|e| {
        log_error(&format!("Error opening local file: {}", e));
        e
    })?;

    io::copy(&mut local_file, &mut remote_file).map_err(|e| {
        log_error(&format!("Error copying file: {}", e));
        e
    })?;

    Ok(())
}

fn generate_unique_filename(original_name: &str) -> String {
    let timestamp = Utc::now().timestamp();
    format!("{}_{}", timestamp, original_name)
}

fn copy_link_to_clipboard(
    file_name: &str,
    settings: &Settings,
) -> Result<(), Box<dyn std::error::Error>> {
    let link = settings.link_template.replace("{file_name}", file_name);
    let mut ctx: ClipboardContext = ClipboardProvider::new().map_err(|e| {
        log_error(&format!("Error creating clipboard context: {}", e));
        e
    })?;

    ctx.set_contents(link).map_err(|e| {
        log_error(&format!("Error copying to clipboard: {}", e));
        e
    })?;

    Ok(())
}

fn show_notification(message: &str) {
    if let Err(e) = Notification::new()
        .summary("File to server sharing")
        .body(message)
        .show()
    {
        log_error(&format!("The notification could not be displayed: {}", e));
    }
}

pub(crate) fn handle_file_sending(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new();

    let original_name = Path::new(file_path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let unique_name = generate_unique_filename(&original_name);

    match send_file(file_path, &unique_name, &settings) {
        Ok(_) => {
            if let Err(e) = copy_link_to_clipboard(&unique_name, &settings) {
                show_notification(&format!("Link copying error: {}", e));
                return Err(e.into());
            } else {
                show_notification("Successfully!\nLink has been copied to the clipboard.");
            }
        }
        Err(e) => {
            show_notification(&format!("File sending error: {}", e));
            return Err(e);
        }
    }

    Ok(())
}
