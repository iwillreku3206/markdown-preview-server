use super::Frame;

/// Data frames sent to the server
#[derive(Debug)]
pub enum EditorServerFrame {
    Ping,
    Pong,
    SetText(String),
    SetDocumentTitle(String),
    SetFilePath(String),
    Close,
}

unsafe impl Send for EditorServerFrame {}

impl Frame for EditorServerFrame {
    fn to_string(&self) -> String {
        match self {
            EditorServerFrame::Ping => "ping".to_string(),
            EditorServerFrame::Pong => "pong".to_string(),
            EditorServerFrame::SetText(text) => format!("set_text {}", text),
            EditorServerFrame::SetDocumentTitle(title) => format!("set_document_title {}", title),
            EditorServerFrame::SetFilePath(path) => format!("set_file_path {}", path),
            EditorServerFrame::Close => "close".to_string(),
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        match self {
            EditorServerFrame::Ping => vec![0x00, 0x01],
            EditorServerFrame::Pong => vec![0x00, 0x02],
            EditorServerFrame::SetText(text) => {
                let mut vec = vec![0x01, 0x00];
                vec.extend_from_slice(text.as_bytes());
                vec
            }
            EditorServerFrame::SetDocumentTitle(title) => {
                let mut vec = vec![0x01, 0x01];
                vec.extend_from_slice(title.as_bytes());
                vec
            }
            EditorServerFrame::SetFilePath(path) => {
                let mut vec = vec![0x01, 0x02];
                vec.extend_from_slice(path.as_bytes());
                vec
            }
            EditorServerFrame::Close => vec![0xff, 0xff],
        }
    }
}
