pub mod frontmatter_parser;
pub mod hooks;
pub mod markdown;
pub mod markdown_extensions;
pub mod util;
pub mod web;

use clap::Parser;
use futures_channel::mpsc::UnboundedSender;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tungstenite::Message;

pub type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

#[derive(Parser, Debug, Clone)]
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
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();

    let sessions = PeerMap::new(Mutex::new(HashMap::new()));

    let _ = tokio::join!(
        tokio::spawn(crate::web::ws::ws_start(sessions.clone(), args.clone())),
        tokio::spawn(crate::web::web_start(sessions.clone(), args.clone()))
    );
}
