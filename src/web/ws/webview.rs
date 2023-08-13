use std::{fs, path::Path};

use tungstenite::Message;

use crate::{web::ws::send_to_all, PeerMaps};

use super::editor::ServerToEditorMessage;

pub async fn handle_incoming(msg: Message, peers: &PeerMaps) -> Result<(), tungstenite::Error> {
    let msg = msg.into_data();
    if msg.len() < 4 {
        return Ok(());
    }

    match &msg[0..4] {
        [0x00, 0x00, 0x01, 0x00] => {
            // move to path
            let path_str = String::from_utf8(msg[4..].to_vec()).unwrap_or_default();
            let path = Path::new(&path_str);
            if path.exists() {
                if let Ok(path_str_canon) = fs::canonicalize(path) {
                    if let Some(path_str_canon_str) = path_str_canon.to_str() {
                        let json = serde_json::to_string(&ServerToEditorMessage::GotoPath(
                            path_str_canon_str.to_string(),
                        ))
                        .unwrap();
                        send_to_all(json.as_bytes().to_vec(), peers.editor_map.clone()).await;
                    }
                }
            }
        }
        _ => {}
    }

    Ok(())
}
