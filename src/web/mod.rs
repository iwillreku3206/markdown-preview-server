mod ping;

use actix_web::{get, App, HttpServer};

#[actix_web::main]
pub async fn web_start() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(ping::ping))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
