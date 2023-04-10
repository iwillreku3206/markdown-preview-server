use std::env;

use futures::future::{ok, select};
use futures_channel::mpsc::unbounded;
use futures_util::{pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};

use crate::PeerMap;

pub async fn ws_start(peers: PeerMap) {
    log::info!("Starting websocket server");
    let _ = env_logger::try_init();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8081".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, peers.clone()));
    }
}

async fn accept_connection(stream: TcpStream, peers: PeerMap) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");

    log::info!("New WebSocket connection: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let (write, read) = ws_stream.split();

    let (tx, rx) = unbounded();
    peers.lock().unwrap().insert(addr, tx);

    let broadcast_incoming = read.try_for_each(|_msg| ok(()));

    let receive_from_others = rx.map(Ok).forward(write);

    pin_mut!(broadcast_incoming, receive_from_others);
    select(broadcast_incoming, receive_from_others).await;
    println!("{} disconnected", &addr);
    peers.lock().unwrap().remove(&addr);
}
