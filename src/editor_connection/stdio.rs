use std::{
    io::{BufRead, BufReader},
    process,
    sync::Arc,
};

use crate::editor_connection::frame::Frame;

use super::{frame::server::ServerFrame, parse_frame::parse_frame, EditorConnection, EditorFrame};

pub struct Stdio {
    cb: Arc<dyn Fn(ServerFrame, &Self) + Send + Sync + 'static>,
}

impl Stdio {
    pub fn new(cb: impl Fn(ServerFrame, &Self) + Send + Sync + 'static) -> Self {
        Stdio { cb: Arc::new(cb) }
    }
}

impl EditorConnection for Stdio {
    fn listen(&self) {
        let stdin = std::io::stdin();
        let mut reader = BufReader::new(stdin);
        let mut buffer = Vec::new();
        loop {
            let bytes_read = reader.read_until(0x0a, &mut buffer).unwrap();
            if bytes_read == 0 {
                continue;
            }
            match parse_frame(&buffer) {
                Some(frame) => match frame {
                    ServerFrame::Close => break,
                    _ => (&self.cb)(frame, &self),
                },
                None => (),
            }
            buffer.clear();
        }
        // If stdio is closed, the process should end
        process::exit(0);
    }

    fn send(&self, data: EditorFrame) {
        println!("{}", &data.to_string());
    }

    fn close(&self) {
        process::exit(0);
    }
}
