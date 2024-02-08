use super::Frame;

/// Data frames sent to the editor
pub enum EditorFrame {
    Ping,
    Pong,
    SetCursorPositon(u64, u64),
    SetFilePath(String),
    Close,
}

impl Frame for EditorFrame {
    fn to_string(&self) -> String {
        match self {
            EditorFrame::Ping => "ping".to_string(),
            EditorFrame::Pong => "pong".to_string(),
            EditorFrame::SetCursorPositon(line, column) => {
                format!("set_cursor_position {} {}", line, column)
            }
            EditorFrame::SetFilePath(path) => format!("set_file_path {}", path),
            EditorFrame::Close => "close".to_string(),
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        match self {
            EditorFrame::Ping => vec![0x00, 0x01],
            EditorFrame::Pong => vec![0x00, 0x02],
            EditorFrame::SetCursorPositon(line, column) => {
                let mut vec = vec![0x01, 0x00];
                vec.extend_from_slice(&line.to_le_bytes());
                vec.extend_from_slice(&column.to_le_bytes());
                vec
            }
            EditorFrame::SetFilePath(path) => {
                let mut vec = vec![0x01, 0x01];
                vec.extend_from_slice(path.as_bytes());
                vec
            }
            EditorFrame::Close => vec![0xff, 0xff],
        }
    }
}
