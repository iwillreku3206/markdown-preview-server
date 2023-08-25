use std::sync::Arc;

use axum::{extract::State, Json};
use futures::lock::Mutex;

use crate::{util::constants::magic_bytes::BYTES_FILENAME, web::ws::send_to_all};

#[derive(serde::Deserialize)]
pub struct DocumentRequest {
    filename: String,
}

#[derive(serde::Serialize)]
pub struct DocumentResponse {
    status: String,
}

pub async fn filename(
    State(state): State<Arc<Mutex<crate::State>>>,
    payload: Json<DocumentRequest>,
) -> Json<DocumentResponse> {
    let unlocked_state = &mut state.lock().await;

    let filename = payload.0.filename.to_string();

    let mut payload: Vec<u8> = Vec::from(BYTES_FILENAME);
    payload.append(&mut filename.clone().as_bytes().to_vec());

    let _ = unlocked_state.set_filename_payload(payload.clone());
    let _ = send_to_all(payload, unlocked_state.sessions.webview_map.clone());

    let result = DocumentResponse {
        status: "ok".to_string(),
    };
    Json(result)
}
