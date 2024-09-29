use std::fs;
use toml::Value;

fn main() {
    // Читаем файл Config.toml на этапе сборки
    let config = fs::read_to_string("Config.toml")
        .expect("Не удалось найти файл конфигурации Config.toml");

    // Парсим конфигурацию как toml
    let config: Value = config.parse().expect("Ошибка парсинга конфигурации");

    // Получаем значения из конфигурации
    let server = config["SERVER"].as_str().unwrap();
    let username = config["USERNAME"].as_str().unwrap();
    let password = config["PASSWORD"].as_str().unwrap();
    let link_template = config["LINK_TEMPLATE"].as_str().unwrap();
    let remote_path = config["REMOTE_PATH"].as_str().unwrap();

    // Встраиваем эти значения в бинарный файл через переменные среды
    println!("cargo:rerun-if-changed=Config.toml");
    println!("cargo:rustc-env=SERVER={}", server);
    println!("cargo:rustc-env=USERNAME={}", username);
    println!("cargo:rustc-env=PASSWORD={}", password);
    println!("cargo:rustc-env=LINK_TEMPLATE={}", link_template);
    println!("cargo:rustc-env=REMOTE_PATH={}", remote_path);
}
