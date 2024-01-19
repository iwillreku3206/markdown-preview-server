use std::{borrow::Cow, net::SocketAddr};

use axum::{
    body::Body,
    extract::{
        ws::{CloseFrame, Message, WebSocket},
        ConnectInfo, WebSocketUpgrade,
    },
    http::Response,
};
use axum_macros::debug_handler;
use futures_util::{SinkExt, StreamExt};

#[debug_handler]
pub async fn viewer_socket_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Response<Body> {
    println!("Viewer connected: {}", addr);
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    //send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        println!("Pinged {who}...");
    } else {
        println!("Could not send ping {who}!");
        return;
    }

    let (mut sender, mut receiver) = socket.split();



    let _ = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            println!("{:?}", msg);
        }
    })
    .await;

    // returning from the handler closes the websocket connection
    println!("Websocket context {who} destroyed");
}
