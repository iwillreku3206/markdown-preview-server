use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use futures::lock::Mutex;

use crate::State;

mod document;
mod filename;
mod frontend;
mod image;
mod ping;
pub mod ws;

pub async fn web_start(state: Arc<Mutex<State>>) {
    let app = Router::new()
        .route("/ping", get(ping::ping))
        .route("/document", post(document::document))
        .route("/filename", post(filename::filename))
        .route("/imagedir/:image", get(image::image))
        .fallback(frontend::frontend)
        .with_state(state.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], state.lock().await.config.port));

    log::info!("Starting server on {}", addr.to_string());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
