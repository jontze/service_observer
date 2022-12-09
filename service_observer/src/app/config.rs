use std::{
    fs::{self, File},
    path::PathBuf,
};

use super::constants::APP_NAME;

pub(crate) fn ensure_app_files_exist() {
    create_config();
    create_database();
}

pub(crate) fn get_db_path() -> PathBuf {
    let mut data_file_path = get_db_dir();
    data_file_path.push("data.sqlite");
    data_file_path
}

pub(crate) fn get_config_path() -> PathBuf {
    let mut config_file_path = get_config_dir();
    config_file_path.push("data.sqlite");
    config_file_path
}

fn get_config_dir() -> PathBuf {
    let mut config_file_path = dirs::config_dir().expect("Unable to get user config path");
    config_file_path.push(&APP_NAME);
    config_file_path
}

fn get_db_dir() -> PathBuf {
    let mut data_file_path = dirs::data_dir().expect("Unable to get user data path");
    data_file_path.push(&APP_NAME);
    data_file_path
}

fn create_config() {
    // Ensure directory exists
    fs::create_dir_all(&get_config_dir()).expect("To create config directory");
    // Ensure file exists
    File::create(&get_config_path()).expect("Config to be created");
}

fn create_database() {
    // Ensure directory exists
    fs::create_dir_all(&get_db_dir()).expect("To create database directory");
    // Ensure file exists
    File::create(&get_db_path()).expect("Database to be created");
}
