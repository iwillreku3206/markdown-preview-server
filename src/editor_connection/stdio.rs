use std::{
    io::{BufRead, BufReader},
    process,
};

use crate::editor_connection::frame::Frame;

use super::{parse_frame::parse_frame, DataCallback, EditorConnection, EditorFrame};

pub struct Stdio {
    cb: DataCallback,
}

impl Stdio {
    pub fn new(cb: DataCallback) -> Self {
        Stdio { cb }
    }
}

impl EditorConnection for Stdio {
    fn listen(&self) {
        let cb = self.cb.clone();
        let stdin = std::io::stdin();
        let mut reader = BufReader::new(stdin);
        let mut buffer = Vec::new();
        loop {
            let bytes_read = reader.read_until(0x0a, &mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            cb(parse_frame(&buffer));
            buffer.clear();
        }
    }

    fn send(&self, data: EditorFrame) {
        println!("{}", &data.to_string());
    }

    fn close(&self) {
        process::exit(0);
    }
}
