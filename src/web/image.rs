/*use serde::Serialize;

#[derive(serde::Deserialize)]
pub struct PingRequest {
    filename: String,
}

#[derive(Serialize)]
pub struct PingResponse {
    pub status: String,
}

pub async fn file() -> Json<PingResponse> {
    let result = PingResponse {
        status: "ok".to_string(),
    };
    Json(result)
}*/
