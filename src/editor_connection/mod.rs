use std::sync::Arc;

use crossbeam::channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use async_trait::async_trait;

use self::frame::{editor::EditorFrame, server::EditorServerFrame};

pub mod frame;
pub mod generic;
pub mod parse_frame;
pub mod stdio;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EditorConnectionType {
    #[serde(rename = "stdio")]
    Stdio,

    #[serde(rename = "websocket")]
    WebSocket,

    #[serde(rename = "ssh")]
    SSH, // TODO: implement SSH
}

#[async_trait]
pub trait EditorConnection: Send + Sync + std::fmt::Debug {
    async fn listen(&self);
    fn send_channel(&self) -> Arc<Mutex<Sender<EditorFrame>>>;
    fn receive_channel(&self) -> Arc<Mutex<Receiver<EditorServerFrame>>>;
    fn send_server_frame_channel(&self) -> Arc<Mutex<Sender<EditorServerFrame>>>;
    fn receive_editor_frame_channel(&self) -> Option<Arc<Mutex<Receiver<EditorFrame>>>>;
    fn close(&self);
}
