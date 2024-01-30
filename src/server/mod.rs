use std::{collections::HashMap, process, sync::Arc};

use markdown_it::MarkdownIt;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod editor;
pub mod parser;
pub mod web;

use crate::{
    args::Args,
    config::Config,
    editor_connection::{
        self, frame::server::ServerFrame, generic::GenericEditorConnection, stdio::Stdio,
        EditorConnection, EditorConnectionType,
    },
    viewer_connection::ViewerMap,
};

use self::editor::Editor;

pub struct Server {
    pub compiler: MarkdownIt,
    //pub editors: HashMap<Uuid, Editor>,
    pub viewers: ViewerMap,
    pub config: Config,
    pub stdio: bool,
    pub io: Arc<dyn EditorConnection>,
}

fn on_editor_close() {
    process::exit(0);
}

impl Server {
    pub fn new(args: &Args, config: Config) -> Self {
        // we can unwrap here since the loader function will always return a connection type
        let connection_type = config
            .editor
            .connection_type
            .clone()
            .unwrap_or(EditorConnectionType::Stdio);
        let io: Arc<dyn EditorConnection> = match connection_type {
            EditorConnectionType::Stdio => Arc::new(Stdio::new()),
            EditorConnectionType::WebSocket => {
                Arc::new(GenericEditorConnection::new(on_editor_close))
            }
            EditorConnectionType::SSH => Arc::new(GenericEditorConnection::new(on_editor_close)),
        };

        Self {
            compiler: MarkdownIt::new(),
            viewers: RwLock::new(HashMap::new()),
            //editors: HashMap::new(),
            config,
            stdio: args.stdio,
            io,
        }
    }

    pub async fn on_frame(self: Arc<Server>, frame: ServerFrame) {
        let io_send = self.io.send_channel().clone();
        match frame {
            editor_connection::frame::server::ServerFrame::Ping => {
                let _ = io_send
                    .lock()
                    .await
                    .send(editor_connection::frame::editor::EditorFrame::Pong)
                    .await
                    .unwrap();
            }
            _ => {}
        };
    }
}
