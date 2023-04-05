mod ping;
mod ws;

use actix_web::{web, App, HttpServer};
use actix::*;

#[actix_web::main]
pub async fn web_start() -> std::io::Result<()> {
    let ws_server = ws::Ws::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ws_server.clone()))
            .service(ping::ping)
            .route("/ws", web::get().to(ws::ws))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
