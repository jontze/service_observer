use std::fs::{self, File};

use super::constants::APP_NAME;

pub(crate) fn ensure_app_files_exist() {
    create_config();
    create_database();
}

fn create_config() {
    let mut config_file_path = dirs::config_dir().expect("Unable to get user config path");
    config_file_path.push(&APP_NAME);
    // Ensure directory exists
    fs::create_dir_all(&config_file_path).expect("To create config directory");
    config_file_path.push("config.toml");
    // Ensure file exists
    File::create(config_file_path).expect("Config to be created");
}

fn create_database() {
    let mut data_file_path = dirs::data_dir().expect("Unable to get user data path");
    data_file_path.push(&APP_NAME);
    // Ensure directory exists
    fs::create_dir_all(&data_file_path).expect("To create database directory");
    data_file_path.push("data.sqlite");
    // Ensure file exists
    File::create(data_file_path).expect("Database to be created");
}
