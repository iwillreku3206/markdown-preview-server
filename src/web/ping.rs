use actix_web::{get, Responder, HttpResponse};
use serde_json::{json};

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "OK"
    }))
}
