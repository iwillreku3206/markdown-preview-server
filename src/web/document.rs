use std::sync::Arc;

use axum::{extract::State, Json};
use futures::lock::Mutex;

use crate::{
    frontmatter_parser::parser::DocumentWithFrontmatter,
    util::constants::magic_bytes::{BYTES_DATA, BYTES_FRONTMATTER},
};

use super::{ws::send_to_all, AppState};

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
    let document_with_frontmatter: DocumentWithFrontmatter =
        crate::frontmatter_parser::parser::parse_file_with_frontmatter(&raw);

    let body = unlocked_state
        .pre_state
        .lock()
        .await
        .current_template
        .get_preview(&markdown, &document_with_frontmatter.frontmatter);

    let mut payload: Vec<u8> = Vec::from(BYTES_DATA);
    payload.append(&mut body.clone().as_bytes().to_vec());

    let _ = unlocked_state.set_content_payload(&payload).await;

    let frontmatter_json =
        serde_json::to_string(&document_with_frontmatter.frontmatter).unwrap_or_default();
    let mut frontmatter_payload: Vec<u8> = Vec::from(BYTES_FRONTMATTER);
    frontmatter_payload.append(&mut frontmatter_json.clone().as_bytes().to_vec());

    let _ = unlocked_state
        .set_frontmatter_payload(&frontmatter_payload)
        .await;

    let _ = send_to_all(
        frontmatter_payload,
        unlocked_state.sessions.webview_map.clone(),
    ).await;

    let _ = send_to_all(
        payload,
        unlocked_state.sessions.webview_map.clone(),
    ).await;

    let result = DocumentResponse {
        status: "ok".to_string(),
    };
    Json(result)
}
