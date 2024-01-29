use std::{collections::HashMap, net::SocketAddr};

use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitSink;
use tokio::sync::{RwLock, Mutex};

pub type ViewerMap = RwLock<HashMap<SocketAddr, Mutex<Viewer>>>;

pub struct Viewer {
    pub addr: SocketAddr,
    pub connection: SplitSink<WebSocket, Message>,
}
