use clap::Parser;
use config::Config;
use editor_connection::{stdio::Stdio, EditorConnection};
use server::Server;

use crate::editor_connection::frame::Frame;

pub mod args;
pub mod config;
pub mod editor_connection;
pub mod error;
pub mod server;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    let config = Config::load(&args).await;
    let io = Stdio::new(|frame| {
        println!("{:?}", frame.to_string());
    });
    let server = Server::new(config);
    let _ = tokio::join!(
        tokio::spawn(async move {
            io.listen();
        }),
        tokio::spawn(async move {
            server.listen_web().await;
        })
    );
}
