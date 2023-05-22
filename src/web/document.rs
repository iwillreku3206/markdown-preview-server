use std::{borrow::BorrowMut, sync::Arc};

use axum::{extract::State, Json};
use futures::lock::Mutex;
use tungstenite::Message;

use crate::util::constants::magic_bytes::BYTES_DATA;

use super::AppState;

#[derive(serde::Deserialize)]
pub struct DocumentRequest {
    text: String,
}

#[derive(serde::Serialize)]
pub struct DocumentResponse {
    status: String,
}

pub async fn document(
    State(state): State<Arc<Mutex<AppState>>>,
    payload: Json<DocumentRequest>,
) -> Json<DocumentResponse> {
    let unlocked_state = &mut state.lock().await;

    let raw = payload.0.text.to_string();

    let markdown: String = crate::markdown::parse_markdown(&raw);

    let mut payload: Vec<u8> = Vec::from(BYTES_DATA);

    payload.append(&mut markdown.clone().as_bytes().to_vec());

    let _ = unlocked_state.set_content_payload(&payload).await;

    let sessions = &unlocked_state.sessions.lock().await;
    let broadcast_recipients = sessions.iter().map(|(_, ws_sink)| ws_sink);
    for recp in broadcast_recipients {
        recp.unbounded_send(Message::Binary(payload.clone()))
            .unwrap();
    }

    let result = DocumentResponse {
        status: "ok".to_string(),
    };
    Json(result)
}
