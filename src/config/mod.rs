use serde::{Deserialize, Serialize};

pub mod loader;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub web: WebConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            web: WebConfig::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebConfig {
    pub port: Option<u16>,
    pub host: Option<String>,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            port: Some(3000),
            host: Some("127.0.0.1".to_string()),
        }
    }
}
