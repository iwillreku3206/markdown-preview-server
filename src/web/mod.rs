use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use futures::lock::Mutex;

use crate::{PeerMap, PreState};

mod document;
mod filename;
mod frontend;
mod ping;
mod image;
pub mod ws;

#[derive(Clone)]
pub struct AppState {
    sessions: PeerMap,
    pre_state: Arc<Mutex<PreState>>,
}

impl AppState {
    pub async fn set_content_payload(&mut self, payload: &Vec<u8>) {
        self.pre_state
            .lock()
            .await
            .set_content_payload(payload.clone())
    }
    pub async fn set_css_payload(&mut self, payload: &Vec<u8>) {
        self.pre_state.lock().await.set_css_payload(payload.clone());
    }
    pub async fn set_filename_payload(&mut self, payload: &Vec<u8>) {
        self.pre_state
            .lock()
            .await
            .set_filename_payload(payload.clone());
    }
    pub async fn set_frontmatter_payload(&mut self, payload: &Vec<u8>) {
        self.pre_state
            .lock()
            .await
            .set_frontmatter_payload(payload.clone());
    }
}

unsafe impl Send for AppState {}

pub async fn web_start(sessions: PeerMap, pre_state: Arc<Mutex<PreState>>) {
    let app = Router::new()
        .route("/ping", get(ping::ping))
        .route("/document", post(document::document))
        .route("/filename", post(filename::filename))
        .fallback(frontend::frontend)
        .with_state(Arc::new(Mutex::new(AppState {
            sessions,
            pre_state: pre_state.clone(),
        })));

    let addr = SocketAddr::from(([127, 0, 0, 1], pre_state.lock().await.config.port));

    log::info!("Starting server on {}", addr.to_string());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
