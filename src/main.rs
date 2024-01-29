use std::sync::Arc;

use clap::Parser;
use config::Config;
use server::web::listen_web;
use server::Server;

pub mod args;
pub mod config;
pub mod editor_connection;
pub mod error;
pub mod server;
pub mod viewer_connection;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = args::Args::parse();
    let config = Config::load(&args).await;

    let server = Arc::new(Server::new(&args, config));

    let io_receive = server.io.receive_channel();

    let server_io_clone = server.clone();
    let server_io_receive_clone = server.clone();
    let server_web_clone = server.clone();

    let _ = tokio::join!(
        tokio::spawn(async move {
            server_io_clone.io.listen().await;
        }),
        tokio::spawn(async move {
            while let Some(frame) = io_receive.lock().await.recv().await {
                server_io_receive_clone.clone().on_frame(frame).await;
            }
        }),
        tokio::spawn(async move {
            listen_web(server_web_clone).await;
        })
    );
}
