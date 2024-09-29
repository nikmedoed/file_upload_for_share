use crate::err_log::log_error;
use std::fs::File;
use std::path::Path;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use chrono::Utc;
use notify_rust::Notification;
use reqwest::blocking::Client;
use std::io::{BufReader};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use base64::engine::general_purpose;
use std::error::Error as StdError;
use base64::Engine as _;

struct Settings {
    username: String,
    password: String,
    link_template: String,
    remote_path: String,
}

impl Settings {
    pub(crate) fn new() -> Self {
        Settings {
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
) -> Result<(), Box<dyn StdError>> {
    let local_file = File::open(file_path).map_err(|e| {
        eprintln!("Error opening local file: {}", e);
        e
    })?;
    let file_reader = BufReader::new(local_file);

    let upload_url = format!("{}/{}", settings.remote_path, remote_filename);

    let auth_header_value = format!(
        "Basic {}",
        general_purpose::STANDARD.encode(format!("{}:{}", settings.username, settings.password))
    );

    let client = Client::new();

    let response = client
        .put(&upload_url)
        .header(AUTHORIZATION, auth_header_value)
        .header(CONTENT_TYPE, "application/octet-stream")
        .body(reqwest::blocking::Body::new(file_reader))
        .send()
        .map_err(|e| {
            eprintln!("Error during file upload: {}", e);
            e
        })?;

    if !response.status().is_success() {
        eprintln!("File upload failed with status: {}", response.status());
        return Err(format!("File upload failed with status: {}", response.status()).into());
    }

    println!("File uploaded successfully.");
    Ok(())
}

fn generate_unique_filename(original_name: &str) -> String {
    let timestamp = Utc::now().timestamp();
    format!("{}_{}", timestamp, original_name.replace(" ", "_"))
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
