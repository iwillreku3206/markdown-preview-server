use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};

use async_trait::async_trait;

use self::frame::{editor::EditorFrame, server::ServerFrame};

pub mod frame;
pub mod parse_frame;
pub mod stdio;

#[async_trait]
pub trait EditorConnection: Send + Sync {
    async fn listen(&self);
    fn send_channel(&self) -> Arc<Mutex<mpsc::Sender<EditorFrame>>>;
    fn receive_channel(&self) -> Arc<Mutex<mpsc::Receiver<ServerFrame>>>;
    fn close(&self);
}
