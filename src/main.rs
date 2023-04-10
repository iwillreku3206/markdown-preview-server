pub mod frontmatter_parser;
pub mod hooks;
pub mod markdown;
pub mod markdown_extensions;
pub mod util;
pub mod web;

use futures_channel::mpsc::UnboundedSender;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tungstenite::Message;

pub type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let sessions = PeerMap::new(Mutex::new(HashMap::new()));

    let _ = tokio::join!(
        tokio::spawn(crate::web::ws::ws_start(sessions.clone())),
        tokio::spawn(crate::web::web_start(sessions.clone()))
    );
}
