use axum::{routing, Router};
use tokio::net::TcpListener;

use crate::config::Config;

pub async fn listen_web(config: &Config) {
    let host = config.web.host.clone().unwrap_or_default();
    let port = config.web.port.clone().unwrap_or_default();
    let _ = tokio::spawn(async move {
        let router = Router::new().route("/", routing::get(|| async { "Hello, world!" }));
        let listener = TcpListener::bind(format!("{}:{}", host, port))
            .await
            .unwrap();
        axum::serve(listener, router).await.unwrap();
    })
    .await;
}
