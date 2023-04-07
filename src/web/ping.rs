use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

#[get("/ping")]
pub async fn ping() -> impl Responder {
    //    println!("{:?}", data);

    HttpResponse::Ok().json(json!({
        "status": "OK"
    }))
}
