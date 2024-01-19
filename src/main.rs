use std::sync::Arc;

use clap::Parser;
use config::Config;
use editor_connection::frame::editor::EditorFrame;
use editor_connection::frame::server::ServerFrame;
use editor_connection::{stdio::Stdio, EditorConnection};
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

    let server_clone = server.clone();
    let io = Stdio::new(move |frame, io| match frame {
        ServerFrame::Ping => io.send(EditorFrame::Pong),
        _ => (),
    });

    let _ = tokio::join!(
        tokio::spawn(async move {
            io.listen();
        }),
        tokio::spawn(async move {
            listen_web(server).await;
        })
    );
}
