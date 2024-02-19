use std::{net::SocketAddr, sync::Arc};

use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    http::Response,
};
use axum_macros::debug_handler;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::Mutex;

use crate::{server::Server, viewer_connection::Viewer};

#[debug_handler]
pub async fn viewer_socket_handler(
    ws: WebSocketUpgrade,
    State(server): State<Arc<Server>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Response<Body> {
    log::info!("Viewer connected: {}", addr);
    ws.on_upgrade(move |socket| handle_socket(socket, addr, server))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr, server: Arc<Server>) {
    //send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        log::info!("Pinged {who}...");
    } else {
        log::info!("Could not send ping {who}!");
        return;
    }

    let (sender, mut receiver) = socket.split();

    let thread = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            log::info!("{:?}", msg);
        }
    });

    let mut viewer = Viewer {
        addr: who,
        connection: sender,
    };

    viewer
        .connection
        .send(Message::Binary([0x00, 0x01].to_vec()))
        .await
        .unwrap();

    server.viewers.write().await.insert(who, Mutex::new(viewer));

    thread.await.unwrap();

    server.viewers.write().await.remove(&who);

    // returning from the handler closes the websocket connection
    log::info!("Websocket context {who} destroyed");
}
