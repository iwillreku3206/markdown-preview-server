use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::{mpsc, Mutex};

use super::{
    frame::{editor::EditorFrame, server::ServerFrame},
    EditorConnection,
};

pub struct GenericEditorConnection {
    send_channel: Arc<Mutex<mpsc::Sender<EditorFrame>>>,
    receive_channel: Arc<Mutex<mpsc::Receiver<ServerFrame>>>,
    send_server_frame_channel: Arc<Mutex<mpsc::Sender<ServerFrame>>>,
    receive_editor_frame_channel: Arc<Mutex<mpsc::Receiver<EditorFrame>>>,
    close_callback: Option<Box<dyn Fn() + Send + Sync>>,
}

#[async_trait]
impl EditorConnection for GenericEditorConnection {
    fn close(&self) {
        if let Some(func) = &self.close_callback {
            func();
        }
    }

    fn send_channel(&self) -> Arc<Mutex<mpsc::Sender<EditorFrame>>> {
        self.send_channel.clone()
    }

    fn receive_channel(&self) -> Arc<Mutex<mpsc::Receiver<ServerFrame>>> {
        self.receive_channel.clone()
    }

    fn send_server_frame_channel(&self) -> Arc<Mutex<mpsc::Sender<ServerFrame>>> {
        self.send_server_frame_channel.clone()
    }

    fn receive_editor_frame_channel(&self) -> Option<Arc<Mutex<mpsc::Receiver<EditorFrame>>>> {
        Some(self.receive_editor_frame_channel.clone())
    }

    async fn listen(&self) {
        return;
    }
}

impl GenericEditorConnection {
    pub fn new(cb: impl Fn() + Send + Sync + 'static) -> Self {
        let (send_editor, receive_editor) = mpsc::channel::<EditorFrame>(16);
        let (send_server, receive_server) = mpsc::channel::<ServerFrame>(16);

        GenericEditorConnection {
            send_channel: Arc::new(Mutex::new(send_editor)),
            receive_channel: Arc::new(Mutex::new(receive_server)),
            send_server_frame_channel: Arc::new(Mutex::new(send_server)),
            receive_editor_frame_channel: Arc::new(Mutex::new(receive_editor)),
            close_callback: Some(Box::new(cb)),
        }
    }
}
