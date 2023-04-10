use axum::{Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct PingResponse {
    pub status: String,
}

pub async fn ping() -> Json<PingResponse> {
    let result = PingResponse {
        status: "ok".to_string(),
    };
    Json(result)
}
