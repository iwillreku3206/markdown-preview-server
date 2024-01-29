use serde::{Deserialize, Serialize};

use crate::editor_connection::EditorConnectionType;

pub mod loader;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub web: WebConfig,
    pub editor: EditorConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            web: WebConfig::default(),
            editor: EditorConfig::default(),
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

#[derive(Debug, Serialize, Deserialize)]
pub struct EditorConfig {
    pub connection_type: Option<EditorConnectionType>,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            connection_type: None, // handle default in loader function
        }
    }
}
