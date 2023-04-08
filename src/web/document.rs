use axum::{extract::State, Json};
use tungstenite::Message;

use crate::PeerMap;

pub struct DocumentRequest {
    text: String,
}

pub struct DocumentResponse {
    status: String,
}

pub async fn document(
    Json(payload): Json<DocumentRequest>,
    State(state): State<PeerMap>,
) -> Json<DocumentResponse> {
    let sessions = state.lock().unwrap();
    let broadcast_recipients = sessions.iter().map(|(_, ws_sink)| ws_sink);

    let raw = payload.text;

    let markdown = crate::markdown::parse_markdown(&raw);

    for recp in broadcast_recipients {
        println!("{:?}", recp);
        recp.unbounded_send(Message::text("markdown".to_string()))
            .unwrap();
    }

    let result = DocumentResponse {
        status: "ok".to_string(),
    };
    Json(result)
}
