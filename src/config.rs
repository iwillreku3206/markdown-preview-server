use std::fs::File;

use serde::{Deserialize, Serialize};

use crate::Args;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub css_dir: String,
    pub template_dir: String,
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
            css_dir: "/etc/markdown-preview-server/style.default.css".to_string(),
            template_dir: "/etc/markdown-preview-server/template.default.html".to_string(),
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
        let mut config = File::open(args.config);
        config.css_dir = args.css;
        config.port = args.port;
        config.websocket_port = args.websocket_port;
        config.frontend_address = args.frontend_address;
        config
    }
}
