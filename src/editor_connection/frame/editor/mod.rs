use super::Frame;

/// Data frames sent to the editor
pub enum EditorFrame {
    Ping,
    Pong,
}

impl Frame for EditorFrame {
    fn to_string(&self) -> String {
        match self {
            EditorFrame::Ping => "ping".to_string(),
            EditorFrame::Pong => "pong".to_string(),
        }
    }
}
