use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::Message;
use futures_util::SinkExt;

use tokio::sync::RwLock;

pub mod editor;
pub mod parser;
pub mod web;

use crate::{
    args::Args,
    config::Config,
    editor_connection::{
        self,
        frame::{server::EditorServerFrame, Frame},
        generic::GenericEditorConnection,
        stdio::Stdio,
        EditorConnection, EditorConnectionType,
    },
    viewer_connection::{frame::viewer::ViewerFrame, ViewerMap},
};

use self::parser::Parser;

pub struct Server {
    pub compiler: Parser,
    //pub editors: HashMap<Uuid, Editor>,
    pub viewers: ViewerMap,
    pub config: Config,
    pub stdio: bool,
    pub io: Arc<dyn EditorConnection>,
}

fn on_editor_close() {}

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
            compiler: Parser::new(),
            viewers: RwLock::new(HashMap::new()),
            config,
            stdio: args.stdio,
            io,
        }
    }

    pub async fn on_frame(self: Arc<Server>, frame: EditorServerFrame) {
        let io_send = self.io.send_channel().clone();
        match frame {
            EditorServerFrame::Ping => {
                io_send
                    .lock()
                    .await
                    .send(editor_connection::frame::editor::EditorFrame::Pong)
                    .unwrap_or(());
            }
            EditorServerFrame::SetText(text) => {
                let html = self.compiler.parse(&text);

                for (_who, viewer) in self.viewers.read().await.iter() {
                    viewer
                        .lock()
                        .await
                        .connection
                        .send(Message::Binary(ViewerFrame::SetText(html.clone()).to_vec()))
                        .await
                        .unwrap();
                }
            }
            EditorServerFrame::SetFilePath(path) => {
                for (_who, viewer) in self.viewers.read().await.iter() {
                    viewer
                        .lock()
                        .await
                        .connection
                        .send(Message::Binary(
                            ViewerFrame::SetFilePath(path.clone()).to_vec(),
                        ))
                        .await
                        .unwrap();
                }
            }
            EditorServerFrame::SetDocumentTitle(title) => {
                for (_who, viewer) in self.viewers.read().await.iter() {
                    viewer
                        .lock()
                        .await
                        .connection
                        .send(Message::Binary(
                            ViewerFrame::SetDocumentTitle(title.clone()).to_vec(),
                        ))
                        .await
                        .unwrap();
                }
            }
            _ => {}
        };
    }
}
