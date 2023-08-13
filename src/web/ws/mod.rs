use std::{net::SocketAddr, sync::Arc};

use futures::{
    future::{ok, select},
    lock::Mutex,
};
use futures_channel::mpsc::unbounded;
use futures_util::{pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

use crate::{PeerMap, PeerMaps, PreState};

pub mod webview;
pub mod editor;

pub async fn ws_start(peers: PeerMaps, pre_state: Arc<Mutex<PreState>>) {
    log::info!("Starting websocket server");
    let addr = format!("127.0.0.1:{}", pre_state.lock().await.config.websocket_port);

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, peers.clone(), pre_state.clone()));
    }
}

async fn accept_connection(stream: TcpStream, peers: PeerMaps, pre_state: Arc<Mutex<PreState>>) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");

    log::info!("New WebSocket connection: {}", addr);

    let mut buf: [u8; 8192] = [0; 8192];
    let _ = &stream.peek(&mut buf).await.unwrap_or_default();
    let mut headers = [httparse::EMPTY_HEADER; 0];

    let mut req = httparse::Request::new(&mut headers);
    let _ = req.parse(&buf).unwrap_or(httparse::Status::Complete(0));

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    match req.path.unwrap_or_default() {
        "/" => {
            handle_webview_ws(addr, ws_stream, peers.webview_map.clone(), peers, pre_state).await
        }
        "/editor" => handle_editor_ws(addr, ws_stream, peers.editor_map.clone(), pre_state).await,
        _ => {}
    }
}

async fn handle_webview_ws(
    addr: SocketAddr,
    stream: WebSocketStream<TcpStream>,
    peers: PeerMap,
    peer_maps: PeerMaps,
    pre_state: Arc<Mutex<PreState>>,
) {
    let (write, read) = stream.split();

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

    let broadcast_incoming = read.try_for_each(|msg| webview::handle_incoming(msg, &peer_maps));
    let receive_from_others = rx.map(Ok).forward(write);

    pin_mut!(broadcast_incoming, receive_from_others);
    select(broadcast_incoming, receive_from_others).await;

    peers.lock().await.remove(&addr);
}

pub async fn send_to_all(message: Vec<u8>, peers: PeerMap) {
    let sessions = &peers.lock().await;
    let broadcast_recipients = sessions.iter().map(|(_, ws_sink)| ws_sink);
    for recp in broadcast_recipients {
        recp.unbounded_send(Message::Binary(message.clone()))
            .unwrap();
    }
}

async fn handle_editor_ws(
    addr: SocketAddr,
    stream: WebSocketStream<TcpStream>,
    peers: PeerMap,
    _pre_state: Arc<Mutex<PreState>>,
) {
    let (write, read) = stream.split();

    let (tx, rx) = unbounded();

    tx.unbounded_send(tungstenite::Message::Binary(
        "EDITOR_WS".as_bytes().to_vec(),
    ))
    .unwrap();

    peers.lock().await.insert(addr, tx);

    let broadcast_incoming = read.try_for_each(|msg| {
        tokio::spawn(send_to_all(msg.into_data(), peers.clone()));
        ok(())
    });
    let receive_from_others = rx.map(Ok).forward(write);

    pin_mut!(broadcast_incoming, receive_from_others);
    select(broadcast_incoming, receive_from_others).await;

    peers.lock().await.remove(&addr);
}
