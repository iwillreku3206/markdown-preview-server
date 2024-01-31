use std::{net::SocketAddr, sync::Arc};

use axum::{routing, Router};
use tokio::net::TcpListener;

pub mod editor_socket;
pub mod static_dir;
pub mod viewer_socket;
pub mod views;

use crate::editor_connection::EditorConnectionType;

use self::{
    editor_socket::editor_socket_handler,
    viewer_socket::viewer_socket_handler,
    views::{content::content_handler, index::index_handler},
};

use super::Server;

pub async fn listen_web(server: Arc<Server>) {
    let host = server.config.web.host.clone().unwrap_or_default();
    let port = server.config.web.port.clone().unwrap_or_default();
    let _ = tokio::spawn(async move {
        let mut router = Router::new()
            .route("/", routing::get(index_handler))
            .route(
                "/static/:path",
                routing::get(static_dir::static_dir_handler),
            )
            .route("/viewer", routing::get(viewer_socket_handler))
            .route("/content", routing::get(content_handler));
        if let Some(EditorConnectionType::WebSocket) = server.config.editor.connection_type {
            router = router.route("/editor", routing::get(editor_socket_handler));
        }
        let router_with_state = router.with_state(server.clone());
        let listener = TcpListener::bind(format!("{}:{}", host, port))
            .await
            .unwrap();
        axum::serve(
            listener,
            router_with_state.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    })
    .await;
}
