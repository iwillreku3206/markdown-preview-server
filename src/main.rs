pub mod config;
mod css;
pub mod frontmatter_parser;
pub mod hooks;
pub mod markdown;
pub mod markdown_extensions;
pub mod patches;
pub mod template;
pub mod util;
pub mod web;

use crate::config::Config;
use clap::Parser;
use css::watch_user_css;
use env_logger::Env;
use futures::lock::Mutex;
use futures_channel::mpsc::UnboundedSender;
use schemars::schema_for;
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use template::PreparedTemplate;
use tungstenite::Message;
use util::constants::magic_bytes::{BYTES_CSS, BYTES_DATA, BYTES_FILENAME, BYTES_FRONTMATTER};

pub type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

#[derive(Debug, Clone)]
pub struct PeerMaps {
    webview_map: PeerMap,
    editor_map: PeerMap,
}

#[cfg(target_os = "linux")]
static DEFAULT_CONFIG_PATH: &str = "/etc/markdown-preview-server/config.toml";

#[cfg(target_os = "windows")]
static DEFAULT_CONFIG_PATH: &str = "C:\\Program Files\\markdown-preview-server\\config.toml";

#[cfg(target_os = "macos")]
static DEFAULT_CONFIG_PATH: &str = "/private/etc/markdown-preview-server/config.toml";

#[derive(Parser, Debug, Clone, Deserialize)]
#[command(author, version, about)]
pub struct Args {
    /// Configuration file path
    #[arg(short, long  = "config-path", value_name = "PATH", default_value = DEFAULT_CONFIG_PATH)]
    pub config_path: String,

    /// Outputs the default configuration into stdout
    #[arg(long = "generate-config-file")]
    pub generate_config_file: bool,

    /// Outputs the template schema into stdout
    #[arg(long = "generate-template-schema")]
    pub generate_template_schema: bool,
}

#[derive(Clone)]
pub struct PreState {
    args: Args,
    config: Config,
    current_content_payload: Vec<u8>,
    current_css_payload: Vec<u8>,
    current_filename_payload: Vec<u8>,
    current_frontmatter_payload: Vec<u8>,
    current_template: PreparedTemplate,
}

impl PreState {
    pub fn set_content_payload(&mut self, payload: Vec<u8>) {
        self.current_content_payload = payload;
    }
    pub fn set_css_payload(&mut self, payload: Vec<u8>) {
        self.current_css_payload = payload;
    }
    pub fn set_filename_payload(&mut self, payload: Vec<u8>) {
        self.current_filename_payload = payload;
    }
    pub fn set_frontmatter_payload(&mut self, payload: Vec<u8>) {
        self.current_frontmatter_payload = payload;
    }
}

unsafe impl Send for PreState {}

#[tokio::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let args = Args::parse();

    if args.generate_config_file {
        toml::to_string_pretty(&Config::default())
            .unwrap()
            .lines()
            .for_each(|line| println!("{}", line));
        return;
    }

    if args.generate_template_schema {
        println!(
            "{}",
            serde_json::to_string_pretty(&schema_for!(template::TemplateMetadata)).unwrap()
        );
        return;
    }

    let config = Config::load(args.clone());

    let css = css::open_user_css(config.clone().css_dir);

    let mut css_payload = BYTES_CSS.to_vec();
    css_payload.append(&mut css.clone().as_bytes().to_vec());

    let pre_state = Arc::new(Mutex::new(PreState {
        args: args.clone(),
        config: config.clone(),
        current_content_payload: BYTES_DATA.to_vec(),
        current_css_payload: css_payload,
        current_filename_payload: BYTES_FILENAME.to_vec(),
        current_frontmatter_payload: BYTES_FRONTMATTER.to_vec(),
        current_template: PreparedTemplate::load("github", config.clone()).unwrap(),
    }));

    let sessions = PeerMaps {
        webview_map: PeerMap::new(Mutex::new(HashMap::new())),
        editor_map: PeerMap::new(Mutex::new(HashMap::new())),
    };

    let _ = tokio::join!(
        tokio::spawn(crate::web::ws::ws_start(
            sessions.clone(),
            pre_state.clone()
        )),
        tokio::spawn(crate::web::web_start(sessions.clone(), pre_state.clone())),
        tokio::spawn(watch_user_css(
            config.css_dir,
            pre_state.clone(),
            sessions.clone(),
        ))
    );
}
