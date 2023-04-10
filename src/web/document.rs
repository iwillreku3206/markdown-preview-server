use axum::{extract::State, Json};
use tungstenite::Message;

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
    State(state): State<AppState>,
    payload: Json<DocumentRequest>,
) -> Json<DocumentResponse> {
    let sessions = state.sessions.lock().unwrap();
    let broadcast_recipients = sessions.iter().map(|(_, ws_sink)| ws_sink);

    let raw = payload.0.text.to_string();

    let mut markdown: String = "<style>".to_string();
    markdown.push_str(&state.pre_state.css);
    markdown.push_str("</style>");
    markdown.push_str(&crate::markdown::parse_markdown(&raw));

    for recp in broadcast_recipients {
        recp.unbounded_send(Message::text(markdown.to_string()))
            .unwrap();
    }

    let result = DocumentResponse {
        status: "ok".to_string(),
    };
    Json(result)
}
