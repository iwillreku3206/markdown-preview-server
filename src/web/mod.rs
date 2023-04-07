mod ping;
pub mod ws;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
pub async fn web_start() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(ping::ping))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
