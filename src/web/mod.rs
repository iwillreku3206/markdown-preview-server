use axum::routing::{get, post_service};
use axum::Router;

use crate::PeerMap;

mod document;
mod ping;
pub mod ws;

pub async fn web_start(sessions: PeerMap) {
    eprintln!("Web server starting...");

    let app = Router::new()
        .route("/ping", get(ping::ping))
        .route("/document", post_service(document::document))
        .with_state(sessions);

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
