use std::sync::Arc;

use axum::{extract::State, Json};
use serde::Serialize;
use tungstenite::Message;

use crate::PeerMap;

#[derive(Serialize)]
pub struct PingResponse {
    pub status: String,
}

pub async fn ping(State(state): State<PeerMap>) -> Json<PingResponse> {
    let sessions = state.lock().unwrap();
    let broadcast_recipients = sessions.iter().map(|(_, ws_sink)| ws_sink);

    for recp in broadcast_recipients {
        println!("{:?}", recp);
        recp.unbounded_send(Message::text("test".to_string()))
            .unwrap();
    }

    let result = PingResponse {
        status: "OK".to_string(),
    };
    Json(result)
}
