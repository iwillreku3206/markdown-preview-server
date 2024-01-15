use super::Frame;

/// Data frames sent to the server
pub enum ServerFrame {
    Ping,
    Pong,
    SetText(String),
    SetDocumentTitle(String),
    SetFilePath(String),
    Close,
}

impl Frame for ServerFrame {
    fn to_string(&self) -> String {
        match self {
            ServerFrame::Ping => "ping".to_string(),
            ServerFrame::Pong => "pong".to_string(),
            ServerFrame::SetText(text) => format!("set_text {}", text),
            ServerFrame::SetDocumentTitle(title) => format!("set_document_title {}", title),
            ServerFrame::SetFilePath(path) => format!("set_file_path {}", path),
            ServerFrame::Close => "close".to_string(),
        }
    }
}
