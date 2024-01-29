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

    let server_io_clone = server.clone();
    let server_web_clone = server.clone();

    let _ = tokio::join!(
        tokio::spawn(async move {
            server_io_clone.listen_io(server_io_clone.clone()).await;
        }),
        tokio::spawn(async move {
            listen_web(server_web_clone).await;
        })
    );
}
