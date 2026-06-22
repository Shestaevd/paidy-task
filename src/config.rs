use config::{Config, Environment, File};
use log::{error, info};
use serde::Deserialize;
use std::path::Path;
use crate::model::error::ConfigReadingError;
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct AppConfig {
    pub http: HttpConfig,
    pub postgres: PostgresConfig,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct HttpConfig {
    pub port: u16,
    pub host: String
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pwd: String,
    pub data_base: String,
    pub pool_size: u32
}

pub fn get_config(
    path: &str,
    env: &str,
    env_vars_prefix: &str,
) -> Result<AppConfig, ConfigReadingError> {
    let config_str: String = path.to_string() + "/base_config.json";
    let config_path: &Path = Path::new(&config_str);
    let config_env_str: String = path.to_string() + &format!("/{env}_config.json");
    let config_env_path: &Path = Path::new(&config_env_str);
    info!(
        "Attempting to load config from: {}",
        config_env_path.display()
    );

    if !config_path.exists() {
        error!("Config file not found at specified path");
        return Err(ConfigReadingError::WrongPathError(config_str));
    }

    let config: AppConfig = read_config_from_path(config_path, config_env_path, env_vars_prefix)?;
    Ok(config)
}

fn read_config_from_path(
    path: &Path,
    env: &Path,
    env_vars_prefix: &str,
) -> Result<AppConfig, ConfigReadingError> {
    Config::builder()
        .add_source(File::from(path))
        // base config will be overwritten by env config.
        .add_source(File::from(env))
        // Env variables to pass sensitive values from Key vault.
        // Even if I am not using it in this project, I still think it will be good to add it
        .add_source(Environment::with_prefix(env_vars_prefix).separator("__"))
        .build()
        .and_then(|json| json.try_deserialize::<AppConfig>())
        .map_err(|e| ConfigReadingError::ParsingError(e.to_string()))
}
