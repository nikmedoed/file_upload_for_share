#![windows_subsystem = "windows"]

mod send_file;
mod err_log;

use err_log::log_error;
use send_file::handle_file_sending;
use std::env;
use winreg::enums::*;
use winreg::RegKey;

fn main() {
    if let Err(e) = add_to_startup() {
        log_error(&format!("Failed to add to startup: {}", e));
    }

    if let Err(e) = add_context_menu() {
        log_error(&format!("Failed to add context menu: {}", e));
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    let file_path = &args[1];

    if let Err(e) = handle_file_sending(file_path) {
        log_error(&format!("Error sending the file: {}", e));
    }
}


fn add_context_menu() -> Result<(), Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let classes_key = hkcu.open_subkey_with_flags("Software\\Classes", KEY_ALL_ACCESS)?;
    let (send_to_server_key, _) = classes_key.create_subkey(r"*\\shell\\SendToServer")?;
    send_to_server_key.set_value("", &"Отправить на сервер")?;
    let (command_key, _) = send_to_server_key.create_subkey("Command")?;
    command_key.set_value("", &format!("\"{}\" \"%1\"", std::env::current_exe()?.display()))?;
    Ok(())
}

fn add_to_startup() -> Result<(), Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_SET_VALUE,
    )?;
    let exe_path = std::env::current_exe()?.display().to_string();
    run_key.set_value("SendToServerApp", &exe_path)?;
    Ok(())
}
