use std::{net::SocketAddr, sync::Arc};

use axum::{routing, Router};
use tokio::net::TcpListener;

pub mod static_dir;
pub mod viewer_socket;
pub mod views;

use self::{viewer_socket::viewer_socket_handler, views::index::index_handler};

use super::Server;

pub async fn listen_web(server: Arc<Server>) {
    let host = server.config.web.host.clone().unwrap_or_default();
    let port = server.config.web.port.clone().unwrap_or_default();
    let _ = tokio::spawn(async move {
        let mut router = Router::new().route("/", routing::get(index_handler)).route(
            "/static/:path",
            routing::get(static_dir::static_dir_handler),
        ).with_state(server.clone());
        if !server.stdio {
            router = router.route("/viewer", routing::get(viewer_socket_handler));
        }
        let listener = TcpListener::bind(format!("{}:{}", host, port))
            .await
            .unwrap();
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    })
    .await;
}
