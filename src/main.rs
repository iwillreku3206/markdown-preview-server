use std::sync::Arc;

use clap::Parser;
use config::Config;
use editor_connection::frame::editor::EditorFrame;
use editor_connection::frame::server::ServerFrame;
use editor_connection::{stdio::Stdio, EditorConnection};
use futures_util::future::IntoFuture;
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

    let io = Arc::new(Stdio::new());

    let io_receive = io.receive_channel().clone();
    let io_send = io.send_channel().clone();

    let server = Arc::new(Server::new(&args, config, io.clone()));

    let _ = tokio::join!(
        tokio::spawn(async move {
            while let Some(frame) = io_receive.lock().await.recv().await {
                match frame {
                    ServerFrame::Ping => {
                        let _ = io_send.lock().await.send(EditorFrame::Pong);
                    }
                    _ => {}
                };
            }
        }),
        tokio::spawn(async move {
            let _ = io.listen().await;
        }),
        tokio::spawn(async move {
            listen_web(server).await;
        })
    );
}
