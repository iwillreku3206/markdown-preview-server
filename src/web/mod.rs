use std::net::{SocketAddr};

use axum::routing::{get, post};
use axum::Router;

use crate::PeerMap;

mod document;
mod ping;
pub mod ws;

pub async fn web_start(sessions: PeerMap, args: crate::Args) {
    let app = Router::new()
        .route("/ping", get(ping::ping))
        .route("/document", post(document::document))
        .with_state(sessions);

    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
