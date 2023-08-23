use std::{
    fs,
    path::{self, Path},
    sync::Arc,
};

use futures::lock::Mutex;
use relative_path::RelativePath;
use tungstenite::Message;

use crate::{web::ws::send_to_all, PeerMaps, PreState};

use super::{editor::ServerToEditorMessage, send_to_all_editors};

pub async fn handle_incoming(
    msg: Message,
    peers: &PeerMaps,
    state: Arc<Mutex<PreState>>,
) -> Result<(), tungstenite::Error> {
    let msg = msg.into_data();
    if msg.len() < 4 {
        return Ok(());
    }

    match &msg[0..4] {
        [0x00, 0x00, 0x01, 0x00] => {
            // move to path
            let path_str = String::from_utf8(msg[4..].to_vec()).unwrap_or_default();
            let current_filename_payload = &state.lock().await.current_filename_payload;
            let current_path_str =
                String::from_utf8(current_filename_payload[4..].to_vec()).unwrap_or_default();
            if let Some(current_path_str_parent) = Path::new(&current_path_str).parent() {
                let rel_path = RelativePath::new(&path_str);

                let _path = rel_path.to_logical_path(&current_path_str_parent);
                let path = Path::new(&_path);
                if path.exists() {
                    if let Ok(path_str_canon) = fs::canonicalize(path) {
                        if let Some(path_str_canon_str) = path_str_canon.to_str() {
                            let json = serde_json::to_string(&ServerToEditorMessage::GotoPath(
                                path_str_canon_str.to_string(),
                            ))
                            .unwrap();
                            send_to_all_editors(json.as_bytes().to_vec(), peers.editor_map.clone())
                                .await;
                        }
                    }
                }
            }
        }
        _ => {}
    }

    Ok(())
}
