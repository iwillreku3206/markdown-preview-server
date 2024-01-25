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

	fn to_vec(&self) -> Vec<u8> {
	    match self {
	        ServerFrame::Ping => vec![0x00, 0x01],
	        ServerFrame::Pong => vec![0x00, 0x02],
	        ServerFrame::SetText(text) => {
	            let mut vec = vec![0x01, 0x00];
	            vec.extend_from_slice(text.as_bytes());
	            vec
	        }
	        ServerFrame::SetDocumentTitle(title) => {
	            let mut vec = vec![0x01, 0x01];
	            vec.extend_from_slice(title.as_bytes());
	            vec
	        }
	        ServerFrame::SetFilePath(path) => {
	            let mut vec = vec![0x01, 0x02];
	            vec.extend_from_slice(path.as_bytes());
	            vec
	        }
	        ServerFrame::Close => vec![0xff, 0xff],
	    }
	}
}
