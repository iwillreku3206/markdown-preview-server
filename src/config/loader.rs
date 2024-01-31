use std::path::{Path, PathBuf};

use directories::BaseDirs;
use log::warn;

use crate::{
    args::Args,
    editor_connection::EditorConnectionType,
    error::{config::NoConfigFileError, AnyError, FileNotFoundError},
};

use super::Config;

async fn try_load_config_file(config_dir_str: Option<String>) -> Result<Config, AnyError> {
    let config_dir_str = config_dir_str.ok_or(NoConfigFileError {})?;
    let config_dir = Path::new(&config_dir_str).to_owned();
    if config_dir.exists() {
        let config_str = tokio::fs::read_to_string(config_dir).await?;
        Ok(toml::from_str::<Config>(&config_str)?)
    } else {
        Err(Box::new(FileNotFoundError {
            path: config_dir.to_str().unwrap_or_default().to_string(),
        }))
    }
}

impl Config {
    pub async fn load(args: &Args) -> Config {
        let config_dir = if let Some(config_dir) = &args.config {
            if let Ok(path) = PathBuf::from(config_dir).canonicalize() {
                Some(path.to_str().unwrap_or_default().to_string())
            } else {
                None
            }
        } else {
            if let Some(base_dirs) = BaseDirs::new() {
                Some(
                    PathBuf::from(
                        base_dirs
                            .config_dir()
                            .to_str()
                            .unwrap_or_default()
                            .to_string(),
                    )
                    .join("markdown-preview-server/config.toml")
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
            Ok(mut config) => {
                if args.stdio {
                    config.editor.connection_type = Some(EditorConnectionType::Stdio);
                }
                config
            }
            Err(e) => {
                warn!("Failed to load config file: {}", e);
                Config::default()
            }
        }
    }
}
