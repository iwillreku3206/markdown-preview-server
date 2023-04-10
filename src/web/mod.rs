use std::net::SocketAddr;

use axum::routing::{get, post};
use axum::Router;

use crate::{PeerMap, PreState};

mod document;
mod ping;
pub mod ws;

#[derive(Clone)]
pub struct AppState {
    sessions: PeerMap,
    pre_state: PreState,
}

pub async fn web_start(sessions: PeerMap, pre_state: PreState) {
    let app = Router::new()
        .route("/ping", get(ping::ping))
        .route("/document", post(document::document))
        .with_state(AppState {
            sessions,
            pre_state: pre_state.clone(),
        });

    let addr = SocketAddr::from(([127, 0, 0, 1], pre_state.args.port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
