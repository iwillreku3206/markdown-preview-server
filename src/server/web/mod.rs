use axum::{routing, Router};
use tokio::net::TcpListener;

use super::Server;

impl Server {
    pub async fn listen_web(&self) {
        let host = self.config.web.host.clone().unwrap_or_default();
        let port = self.config.web.port.clone().unwrap_or_default();
        let _ = tokio::spawn(async move {
            let router = Router::new().route("/", routing::get(|| async { "Hello, world!" }));
            let listener = TcpListener::bind(format!("{}:{}", host, port))
                .await
                .unwrap();
            axum::serve(listener, router).await.unwrap();
        })
        .await;
    }
}
