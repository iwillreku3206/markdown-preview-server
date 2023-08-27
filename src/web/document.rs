use std::sync::Arc;

use axum::{extract::State, Json};
use futures::lock::Mutex;

use crate::{
    util::constants::magic_bytes::{BYTES_DATA, BYTES_FRONTMATTER},
    web::ws::send_to_all,
};

#[derive(serde::Deserialize)]
pub struct DocumentRequest {
    text: String,
    editor_id: String,
    request_number: u64,
}

#[derive(serde::Serialize)]
pub struct DocumentResponse {
    status: String,
}

pub async fn document(
    State(state): State<Arc<Mutex<crate::State>>>,
    payload: Json<DocumentRequest>,
) -> Json<DocumentResponse> {
    let unlocked_state = &mut state.lock().await;

    let raw = payload.0.text.to_string();
    if unlocked_state.current_editor == payload.0.editor_id
        && unlocked_state.current_request_number > payload.0.request_number
    {
        return Json(DocumentResponse {
            status: "old".to_string(),
        });
    }

    unlocked_state.current_editor = payload.0.editor_id;
    unlocked_state.current_request_number = payload.0.request_number;

    let (markdown, frontmatter) = unlocked_state.parser.parse(&raw);

    let body = unlocked_state
        .current_template
        .get_preview(&markdown, &frontmatter);

    let mut payload: Vec<u8> = Vec::from(BYTES_DATA);
    payload.append(&mut body.clone().as_bytes().to_vec());

    let _ = unlocked_state.set_content_payload(payload.clone());

    let frontmatter_json = serde_json::to_string(&frontmatter).unwrap_or_default();
    let mut frontmatter_payload: Vec<u8> = Vec::from(BYTES_FRONTMATTER);
    frontmatter_payload.append(&mut frontmatter_json.clone().as_bytes().to_vec());

    let _ = unlocked_state.set_frontmatter_payload(frontmatter_payload.clone());

    unlocked_state.current_document = raw.clone();

    let _ = send_to_all(
        frontmatter_payload,
        unlocked_state.sessions.webview_map.clone(),
    )
    .await;

    let _ = send_to_all(payload, unlocked_state.sessions.webview_map.clone()).await;

    let result = DocumentResponse {
        status: "ok".to_string(),
    };
    Json(result)
}
