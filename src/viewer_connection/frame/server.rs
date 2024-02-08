use crate::editor_connection::frame::Frame;

/// Data frame to be sent to the server (from viewer)
pub enum ViewerServerFrame {
    Ping,
    Pong,
    Close,
}

impl Frame for ViewerServerFrame {
    fn to_string(&self) -> String {
        match self {
            ViewerServerFrame::Ping => "ping".to_string(),
            ViewerServerFrame::Pong => "pong".to_string(),
            ViewerServerFrame::Close => "close".to_string(),
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        match self {
            ViewerServerFrame::Ping => vec![0x00, 0x01],
            ViewerServerFrame::Pong => vec![0x00, 0x02],
            ViewerServerFrame::Close => vec![0xff, 0xff],
        }
    }
}
