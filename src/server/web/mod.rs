use axum::{routing, Router};
use tokio::net::TcpListener;

pub mod static_dir;
pub mod views;

use crate::config::Config;

use self::views::index::{index_handler};

pub async fn listen_web(config: &Config) {
    let host = config.web.host.clone().unwrap_or_default();
    let port = config.web.port.clone().unwrap_or_default();
    let _ = tokio::spawn(async move {
        let router = Router::new().route("/", routing::get(index_handler)).route(
            "/static/:path",
            routing::get(static_dir::static_dir_handler),
        );
        let listener = TcpListener::bind(format!("{}:{}", host, port))
            .await
            .unwrap();
        axum::serve(listener, router).await.unwrap();
    })
    .await;
}
