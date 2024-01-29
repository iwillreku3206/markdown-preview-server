use std::{io::BufRead, process, sync::Arc};

use async_trait::async_trait;
use tokio::sync::{mpsc, Mutex};

use std::io;

use crate::editor_connection::frame::Frame;

use super::{frame::server::ServerFrame, parse_frame::parse_frame, EditorConnection, EditorFrame};

pub struct Stdio {
    send_channel: Arc<Mutex<mpsc::Sender<EditorFrame>>>,
    receive_channel: Arc<Mutex<mpsc::Receiver<ServerFrame>>>,
    send_server_frame_channel: Arc<Mutex<mpsc::Sender<ServerFrame>>>,
}

impl Stdio {
    pub fn new() -> Self {
        let (send_editor, mut receive_editor) = mpsc::channel::<EditorFrame>(16);
        let (send_server, receive_server) = mpsc::channel::<ServerFrame>(16);

        tokio::spawn(async move {
            // process incoming server frames here
            while let Some(frame) = receive_editor.recv().await {
                println!("{}", frame.to_string());
            }
        });

        Stdio {
            send_channel: Arc::new(Mutex::new(send_editor)),
            receive_channel: Arc::new(Mutex::new(receive_server)),
            send_server_frame_channel: Arc::new(Mutex::new(send_server)),
        }
    }
}

#[async_trait]
impl EditorConnection for Stdio {
    async fn listen(&self) {
        let mut buf = Vec::new();
        while let Ok(_) = io::stdin().lock().read_until(b'\n', &mut buf) {
            match parse_frame(&buf) {
                Some(frame) => {
                    match frame {
                        ServerFrame::Close => break,
                        _ => {
                            println!("frame: {}", frame.to_string());
                            let _ = async {
                                let _ = &self
                                    .send_server_frame_channel
                                    .lock()
                                    .await
                                    .send(frame)
                                    .await;
                            };
                        }
                    };
                }
                None => (),
            };
            buf.clear();
        }
        // If stdio is closed, the process should end
        process::exit(0);
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
        None
    }

    fn close(&self) {
        process::exit(0);
    }
}
