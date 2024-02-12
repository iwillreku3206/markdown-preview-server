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

use crate::{
    editor_connection::{frame::Frame, parse_frame::parse_frame},
    server::Server,
};

#[debug_handler]
pub async fn editor_socket_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(server): State<Arc<Server>>,
) -> Response<Body> {
    log::info!("Editor connected: {}", addr);
    ws.on_upgrade(move |socket| handle_socket(socket, addr, server))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr, server: Arc<Server>) {
    //send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        log::info!("Pinged {who}...");
    } else {
        log::error!("Could not send ping {who}!");
        return;
    }

    let (mut sender, mut receiver) = socket.split();

    let server_clone = server.clone();
    let _ = tokio::join!(
        tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                if let Some(frame) = parse_frame(msg.into_data().as_slice()) {
                    if let Err(err) = server_clone
                        .io
                        .send_server_frame_channel()
                        .lock()
                        .await
                        .send(frame)
                        .await
                    {
                        log::error!("Error sending frame to server: {}", err);
                    };
                }
            }
        }),
        tokio::spawn(async move {
            if let Some(channel) = server.io.receive_editor_frame_channel() {
                while let Ok(frame) = channel.lock().await.try_recv() {
                    if let Err(e) = sender.send(Message::Binary(frame.to_vec())).await {
                        log::error!("Error sending frame to editor: {}", e);
                        break;
                    }
                }
            }
        })
    );

    // returning from the handler closes the websocket connection
    log::info!("Websocket context {who} destroyed");
}
