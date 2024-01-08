use super::Frame;

/// Data frames sent to the server
pub enum ServerFrame {
    Ping,
    Pong,
}

impl Frame for ServerFrame {
    fn to_string(&self) -> String {
        match self {
            ServerFrame::Ping => "ping".to_string(),
            ServerFrame::Pong => "pong".to_string(),
        }
    }
}
