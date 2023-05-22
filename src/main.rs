mod css;
pub mod frontmatter_parser;
pub mod hooks;
pub mod markdown;
pub mod markdown_extensions;
pub mod patches;
pub mod util;
pub mod web;

use clap::Parser;
use env_logger::Env;
use futures::lock::Mutex;
use futures_channel::mpsc::UnboundedSender;
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tungstenite::Message;
use util::constants::magic_bytes::{BYTES_CSS, BYTES_DATA};

pub type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

#[derive(Parser, Debug, Clone, Deserialize)]
#[command(author, version, about)]
pub struct Args {
    /// Path to user-defined CSS file
    #[arg(
        long,
        value_name = "PATH",
        default_value = "/etc/markdown-preview-server/style.default.css"
    )]
    css: String,

    /// Port to listen on
    #[arg(long, short, value_name = "PORT", default_value = "8080")]
    port: u16,

    #[arg(long = "websocket-port", value_name = "PORT", default_value = "8081")]
    websocket_port: u16,

    /// (For development) Address to connect to frontend
    #[arg(long = "frontend-address", value_name = "ADDRESS", default_value = "")]
    frontend_address: String,
}

#[derive(Clone)]
pub struct PreState {
    args: Args,
    current_content_payload: Vec<u8>,
    current_css_payload: Vec<u8>,
}

impl PreState {
    pub fn set_content_payload(&mut self, payload: Vec<u8>) {
        self.current_content_payload = payload;
    }
    pub fn set_css_payload(&mut self, payload: Vec<u8>) {
        self.current_css_payload = payload;
    }
}

unsafe impl Send for PreState {}

#[tokio::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let args = Args::parse();

    let css = css::open_user_css(args.clone().css);

    let mut css_payload = BYTES_CSS.to_vec();
    css_payload.append(&mut css.clone().as_bytes().to_vec());

    let pre_state = Arc::new(Mutex::new(PreState {
        args: args.clone(),
        current_content_payload: BYTES_DATA.to_vec(),
        current_css_payload: css_payload,
    }));

    let sessions = PeerMap::new(Mutex::new(HashMap::new()));

    let _ = tokio::join!(
        tokio::spawn(crate::web::ws::ws_start(
            sessions.clone(),
            pre_state.clone()
        )),
        tokio::spawn(crate::web::web_start(sessions.clone(), pre_state.clone()))
    );
}
