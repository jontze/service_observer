use super::constants::APP_NAME;
use config::{Config, Environment};
use ratatui::style::Color;
use serde::Deserialize;
use std::{
    fs::{self, File},
    io::ErrorKind,
    path::PathBuf,
};

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
    config_file_path.push("config.toml");
    config_file_path
}

fn get_config_dir() -> PathBuf {
    let mut config_file_path = dirs::config_dir().expect("Unable to get user config path");
    config_file_path.push(APP_NAME);
    config_file_path
}

fn get_db_dir() -> PathBuf {
    let mut data_file_path = dirs::data_dir().expect("Unable to get user data path");
    data_file_path.push(APP_NAME);
    data_file_path
}

fn create_config() {
    // Ensure directory exists
    fs::create_dir_all(get_config_dir()).expect("To create config directory");
    // Ensure file exists
    match File::options()
        .read(true)
        .write(true)
        .create_new(true)
        .open(get_config_path())
    {
        Ok(_) => {}
        Err(err) => match err.kind() {
            ErrorKind::AlreadyExists => {}
            _ => panic!("Error during config file creation"),
        },
    }
}

fn create_database() {
    // Ensure directory exists
    fs::create_dir_all(get_db_dir()).expect("To create database directory");
    // Ensure file exists
    match File::options()
        .read(true)
        .write(true)
        .create_new(true)
        .open(get_db_path())
    {
        Ok(_) => {}
        Err(err) => match err.kind() {
            ErrorKind::AlreadyExists => {}
            _ => panic!("Error during database creation"),
        },
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    #[serde(default)]
    pub ui: Ui,
    pub crawler: Crawler,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Ui {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub accent_color: Color,
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            primary_color: Color::White,
            secondary_color: Color::DarkGray,
            accent_color: Color::Red,
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Crawler {
    pub shodan_token: String,
}

impl Settings {
    pub fn new() -> Self {
        let conf = Config::builder()
            .add_source(config::File::with_name(get_config_path().to_str().unwrap()))
            .add_source(Environment::with_prefix(APP_NAME))
            .build()
            .expect("To build config");
        conf.try_deserialize().expect("Failed to parse config file")
    }
}
