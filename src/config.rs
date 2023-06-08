use serde::{Deserialize, Serialize};

use crate::Args;

#[cfg(target_os = "linux")]
static DEFAULT_CSS_DIR: &str = "/etc/markdown-preview-server/style.default.css";
#[cfg(target_os = "linux")]
static DEFAULT_TEMPLATE_DIR: &str = "/etc/markdown-preview-server/templates.d";
#[cfg(target_os = "linux")]
static DEFAULT_IMAGE_DIR: &str = "$HOME/.markdown-preview-server/images";

#[cfg(target_os = "windows")]
static DEFAULT_CSS_DIR: &str = "C:\\Program Files\\markdown-preview-server\\style.default.css";
#[cfg(target_os = "windows")]
static DEFAULT_TEMPLATE_DIR: &str = "C:\\Program Files\\markdown-preview-server\\templates";
#[cfg(target_os = "windows")]
static DEFAULT_IMAGE_DIR: &str = "$APPDATA\\.markdown-preview-server\\images";

#[cfg(target_os = "macos")]
static DEFAULT_CSS_DIR: &str = "/private/etc/markdown-preview-server/style.default.css";
#[cfg(target_os = "macos")]
static DEFAULT_TEMPLATE_DIR: &str = "/private/etc/markdown-preview-server/templates.d";
#[cfg(target_os = "macos")]
static DEFAULT_IMAGE_DIR: &str = "$HOME/.markdown-preview-server/images";

static DEFAULT_TEMPLATE: &str = "default";

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub css_dir: String,
    pub image_dir_enabled: bool,
    pub image_dir: String,
    pub template_dir: String,
    pub template: String,
    pub feature_set: String,
    pub log_level: String,
    pub host: String,
    pub port: u16,
    pub websocket_port: u16,
    pub frontend_address: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            css_dir: DEFAULT_CSS_DIR.to_string(),
            image_dir_enabled: true,
            image_dir: DEFAULT_IMAGE_DIR.to_string(),
            template_dir: DEFAULT_TEMPLATE_DIR.to_string(),
            template: DEFAULT_TEMPLATE.to_string(),
            feature_set: "default".to_string(),
            log_level: "info".to_string(),
            host: "127.0.0.1".to_string(),
            port: 8080,
            websocket_port: 8081,
            frontend_address: "".to_string(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn load(args: Args) -> Self {
        match std::fs::read(args.clone().config_path) {
            Ok(file) => match String::from_utf8(file) {
                Ok(file) => {
                    let config: Config = toml::from_str(&file).unwrap_or_default();
                    config
                }
                Err(_) => Config::default(),
            },
            Err(_) => {
                log::info!("No config file detected at {}", args.config_path);
                Config::default()
            }
        }
    }
}
