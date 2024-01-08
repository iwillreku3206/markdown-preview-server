use std::path::Path;

use directories::BaseDirs;
use log::warn;

use crate::{
    args::Args,
    error::{config::NoConfigFileError, AnyError, FileNotFoundError},
};

use super::Config;

async fn try_load_config_file(config_dir_str: Option<String>) -> Result<Config, AnyError> {
    let config_dir_str = config_dir_str.ok_or(NoConfigFileError {})?;
    let config_dir = Path::new(&config_dir_str);
    let config_file = config_dir.join("markdown-preview-server/config.toml");
    if config_file.exists() {
        let config_str = tokio::fs::read_to_string(config_file).await?;
        Ok(toml::from_str::<Config>(&config_str)?)
    } else {
        Err(Box::new(FileNotFoundError {
            path: config_file.to_str().unwrap_or_default().to_string(),
        }))
    }
}

impl Config {
    pub async fn load(args: &Args) -> Config {
        let config_dir = if let Some(config_dir) = &args.config {
            Some(config_dir.to_string())
        } else {
            if let Some(base_dirs) = BaseDirs::new() {
                Some(
                    base_dirs
                        .config_dir()
                        .to_str()
                        .unwrap_or_default()
                        .to_string(),
                )
            } else {
                None
            }
        };

        let config_file = try_load_config_file(config_dir);

        match config_file.await {
            Ok(config) => config,
            Err(e) => {
                warn!("Failed to load config file: {}", e);
                Config::default()
            }
        }
    }
}

