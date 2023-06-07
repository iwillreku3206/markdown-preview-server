use std::sync::Arc;

use futures::{
    future::{ok, select},
    lock::Mutex,
};
use futures_channel::mpsc::unbounded;
use futures_util::{pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};

use crate::{PeerMap, PreState};

pub async fn ws_start(peers: PeerMap, pre_state: Arc<Mutex<PreState>>) {
    log::info!("Starting websocket server");
    let addr = format!("127.0.0.1:{}", pre_state.lock().await.config.websocket_port);

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, peers.clone(), pre_state.clone()));
    }
}

async fn accept_connection(stream: TcpStream, peers: PeerMap, pre_state: Arc<Mutex<PreState>>) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");

    log::info!("New WebSocket connection: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let (write, read) = ws_stream.split();

    let (tx, rx) = unbounded();

    tx.unbounded_send(tungstenite::Message::Binary(
        pre_state.lock().await.current_content_payload.clone(),
    ))
    .unwrap();

    tx.unbounded_send(tungstenite::Message::Binary(
        pre_state.lock().await.current_css_payload.clone(),
    ))
    .unwrap();

    tx.unbounded_send(tungstenite::Message::Binary(
        pre_state.lock().await.current_frontmatter_payload.clone(),
    ))
    .unwrap();

    tx.unbounded_send(tungstenite::Message::Binary(
        pre_state.lock().await.current_filename_payload.clone(),
    ))
    .unwrap();

    peers.lock().await.insert(addr, tx);

    let broadcast_incoming = read.try_for_each(|_msg| ok(()));
    let receive_from_others = rx.map(Ok).forward(write);

    pin_mut!(broadcast_incoming, receive_from_others);
    select(broadcast_incoming, receive_from_others).await;

    peers.lock().await.remove(&addr);
}
