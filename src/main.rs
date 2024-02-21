use std::sync::Arc;

use crate::editor_connection::frame::Frame;
use clap::Parser;
use config::Config;

use generate_defaults::generate_defaults;
use server::web::listen_web;
use server::Server;

pub mod args;
pub mod cli_compile;
pub mod config;
pub mod editor_connection;
pub mod error;
pub mod generate_defaults;
pub mod markdown_extensions;
pub mod server;
pub mod util;
pub mod viewer_connection;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = args::Args::parse();
    generate_defaults(&args);

    let config = Config::load(&args).await;
    if args.print_config {
        println!("{:#?}", config);
        std::process::exit(0);
    }

    if let Some(ref path) = args.compile_file {
        return cli_compile::cli_compile(&path, &args, config);
    }

    let server = Arc::new(Server::new(&args, config));

    let server_io_clone = server.clone();
    let server_io_receive_clone = server.clone();
    let server_web_clone = server.clone();

    let _ = tokio::join!(
        tokio::spawn(async move {
            server_io_clone.io.listen().await;
        }),
        tokio::spawn(async move {
            let io_receive = server_io_receive_clone.io.receive_channel();
            let channel_lock = io_receive.lock().await;

            while let Ok(frame) = channel_lock.recv() {
                server_io_receive_clone.clone().on_frame(frame).await;
            }
        }),
        tokio::spawn(async move {
            listen_web(server_web_clone).await;
        })
    );
}
