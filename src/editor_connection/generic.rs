use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use tokio::sync::{mpsc, Mutex};

use super::{
    frame::{editor::EditorFrame, server::EditorServerFrame},
    EditorConnection,
};

pub struct GenericEditorConnection {
    send_channel: Arc<Mutex<mpsc::Sender<EditorFrame>>>,
    receive_channel: Arc<Mutex<mpsc::Receiver<EditorServerFrame>>>,
    send_server_frame_channel: Arc<Mutex<mpsc::Sender<EditorServerFrame>>>,
    receive_editor_frame_channel: Arc<Mutex<mpsc::Receiver<EditorFrame>>>,
    close_callback: Option<Box<dyn Fn() + Send + Sync>>,
}

impl Debug for GenericEditorConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericEditorConnection")
            .field("send_channel", &self.send_channel)
            .field("receive_channel", &self.receive_channel)
            .field("send_server_frame_channel", &self.send_server_frame_channel)
            .field(
                "receive_editor_frame_channel",
                &self.receive_editor_frame_channel,
            )
            .finish()
    }
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

    fn receive_channel(&self) -> Arc<Mutex<mpsc::Receiver<EditorServerFrame>>> {
        self.receive_channel.clone()
    }

    fn send_server_frame_channel(&self) -> Arc<Mutex<mpsc::Sender<EditorServerFrame>>> {
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
        let (send_server, receive_server) = mpsc::channel::<EditorServerFrame>(16);

        GenericEditorConnection {
            send_channel: Arc::new(Mutex::new(send_editor)),
            receive_channel: Arc::new(Mutex::new(receive_server)),
            send_server_frame_channel: Arc::new(Mutex::new(send_server)),
            receive_editor_frame_channel: Arc::new(Mutex::new(receive_editor)),
            close_callback: Some(Box::new(cb)),
        }
    }
}
