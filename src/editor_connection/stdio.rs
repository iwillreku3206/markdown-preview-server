use std::{
    io::{BufRead, BufReader},
    process,
    sync::Arc,
};

use async_trait::async_trait;
use tokio::sync::{mpsc, Mutex};

use crate::editor_connection::frame::Frame;

use super::{frame::server::ServerFrame, parse_frame::parse_frame, EditorConnection, EditorFrame};

pub struct Stdio {
    send_channel: Arc<Mutex<mpsc::Sender<EditorFrame>>>,
    receive_channel: Arc<Mutex<mpsc::Receiver<ServerFrame>>>,
    send_server_frame_channel: Arc<Mutex<mpsc::Sender<ServerFrame>>>,
}

impl Stdio {
    pub fn new() -> Self {
        let (mut send_editor, mut receive_editor) = mpsc::channel::<EditorFrame>(16);
        let (mut send_server, mut receive_server) = mpsc::channel::<ServerFrame>(16);

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
        let stdin = std::io::stdin();
        let mut reader = BufReader::new(stdin);
        let mut buffer = Vec::new();
        loop {
            let bytes_read = reader.read_until(0x0a, &mut buffer).unwrap();
            if bytes_read == 0 {
                continue;
            }
            match parse_frame(&buffer) {
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
                            }
                            .await;
                        }
                    };
                }
                None => (),
            }
            buffer.clear();
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

    fn close(&self) {
        process::exit(0);
    }
}
