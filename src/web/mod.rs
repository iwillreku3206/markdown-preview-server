use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use tokio::sync::broadcast;

use crate::PeerMap;

mod ping;
mod document;
pub mod ws;

pub async fn web_start(sessions: PeerMap) {
    eprintln!("Web server starting...");

    let app = Router::new()
        .route("/ping", get(ping::ping))
        .with_state(sessions);

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
