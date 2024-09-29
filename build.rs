use std::fs;
use toml::Value;

fn main() {
    let config = fs::read_to_string("Config.toml")
        .expect("Не удалось найти файл конфигурации Config.toml");

    let config: Value = config.parse().expect("Ошибка парсинга конфигурации");

    let username = config["USERNAME"].as_str().unwrap();
    let password = config["PASSWORD"].as_str().unwrap();
    let link_template = config["LINK_TEMPLATE"].as_str().unwrap();
    let remote_path = config["REMOTE_PATH"].as_str().unwrap();

    println!("cargo:rerun-if-changed=Config.toml");
    println!("cargo:rustc-env=USERNAME={}", username);
    println!("cargo:rustc-env=PASSWORD={}", password);
    println!("cargo:rustc-env=LINK_TEMPLATE={}", link_template);
    println!("cargo:rustc-env=REMOTE_PATH={}", remote_path);
}
