use crate::editor_connection::frame::Frame;

/// Data frame to be sent to the viewer
pub enum ViewerFrame {
    Ping,
    Pong,
    SetText(String),
    SetDocumentTitle(String),
    SetFilePath(String),
    SetCursorPositon(u64, u64),
    Close,
}

impl Frame for ViewerFrame {
    fn to_string(&self) -> String {
        match self {
            ViewerFrame::Ping => "ping".to_string(),
            ViewerFrame::Pong => "pong".to_string(),
            ViewerFrame::SetText(text) => format!("set_text {}", text),
            ViewerFrame::SetDocumentTitle(title) => format!("set_document_title {}", title),
            ViewerFrame::SetFilePath(path) => format!("set_file_path {}", path),
            ViewerFrame::SetCursorPositon(x, y) => format!("set_cursor_position {} {}", x, y),
            ViewerFrame::Close => "close".to_string(),
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        match self {
            ViewerFrame::Ping => vec![0x00, 0x01],
            ViewerFrame::Pong => vec![0x00, 0x02],
            ViewerFrame::SetText(text) => {
                let mut vec = vec![0x01, 0x00];
                vec.extend_from_slice(text.as_bytes());
                vec
            }
            ViewerFrame::SetDocumentTitle(title) => {
                let mut vec = vec![0x01, 0x01];
                vec.extend_from_slice(title.as_bytes());
                vec
            }
            ViewerFrame::SetFilePath(path) => {
                let mut vec = vec![0x01, 0x02];
                vec.extend_from_slice(path.as_bytes());
                vec
            }
            ViewerFrame::SetCursorPositon(x, y) => {
                let mut vec = vec![0x01, 0x03];
                vec.extend_from_slice(&x.to_be_bytes());
                vec.extend_from_slice(&y.to_be_bytes());
                vec
            }
            ViewerFrame::Close => vec![0xff, 0xff],
        }
    }
}
