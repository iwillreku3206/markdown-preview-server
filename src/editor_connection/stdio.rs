use std::{
    io::{self, BufRead, BufReader},
    process,
    sync::Arc,
};

use async_trait::async_trait;
use crossbeam::channel::{unbounded, Receiver, Sender};

use tokio::sync::Mutex;

use super::{
    frame::server::EditorServerFrame, frame::Frame, parse_frame::parse_frame, EditorConnection,
    EditorFrame,
};

#[derive(Debug)]
pub struct Stdio {
    send_channel: Arc<Mutex<Sender<EditorFrame>>>,
    receive_channel: Arc<Mutex<Receiver<EditorServerFrame>>>,
    send_server_frame_channel: Arc<Mutex<Sender<EditorServerFrame>>>,
}

impl Stdio {
    pub fn new() -> Self {
        let (send_editor, receive_editor) = unbounded::<EditorFrame>();
        let (send_server, receive_server) = unbounded::<EditorServerFrame>();

        tokio::spawn(async move {
            // process incoming server frames here
            while let Ok(_frame) = receive_editor.recv() {
                // println!("{}", frame.to_string());
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
        let mut stdin = io::stdin();
        let mut reader = BufReader::new(&mut stdin);
        'loop1: while let Ok(_) = reader.read_until(b'\n', &mut buf) {
            let frame = parse_frame(&buf);
            match frame {
                Some(frame) => {
                    match frame {
                        EditorServerFrame::Close => break 'loop1,
                        _ => match &self.send_server_frame_channel.lock().await.send(frame) {
                            Ok(_) => (),
                            Err(_) => break 'loop1,
                        },
                    };
                }
                None => (),
            };
            buf.clear();
        }
        // If stdio is closed, the process should end
        process::exit(0);
    }

    fn send_channel(&self) -> Arc<Mutex<Sender<EditorFrame>>> {
        self.send_channel.clone()
    }

    fn receive_channel(&self) -> Arc<Mutex<Receiver<EditorServerFrame>>> {
        self.receive_channel.clone()
    }

    fn send_server_frame_channel(&self) -> Arc<Mutex<Sender<EditorServerFrame>>> {
        self.send_server_frame_channel.clone()
    }

    fn receive_editor_frame_channel(&self) -> Option<Arc<Mutex<Receiver<EditorFrame>>>> {
        None
    }

    fn close(&self) {
        process::exit(0);
    }
}
