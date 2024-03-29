use std::{net::SocketAddr, sync::Arc};

use futures::future::{ok, select};
use futures_channel::mpsc::unbounded;
use futures_util::{lock::Mutex, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

use crate::{web::ws::editor::ServerToEditorMessage, EditorMap, PeerMap, PeerMaps};
use uuid::Uuid;

pub mod editor;
pub mod webview;

pub async fn ws_start(state: Arc<Mutex<crate::State>>) {
    log::info!("Starting websocket server");
    let addr = format!("127.0.0.1:{}", state.lock().await.config.websocket_port);

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, state.clone()));
    }
}

async fn accept_connection(stream: TcpStream, state: Arc<Mutex<crate::State>>) -> impl Send {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");

    let mut buf: [u8; 8192] = [0; 8192];
    let _ = &stream.peek(&mut buf).await.unwrap_or_default();
    let mut headers = [httparse::EMPTY_HEADER; 0];

    let mut req = httparse::Request::new(&mut headers);
    let _ = req.parse(&buf).unwrap_or(httparse::Status::Complete(0));

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let path = req.path.unwrap_or_default();
    let peers = state.lock().await.sessions.clone();

    log::info!("New WebSocket connection ({path}): {addr}. There were {} webview and {} editor connections.", peers.webview_map.lock().await.len(), peers.editor_map.lock().await.len());

    match path {
        "/" => {
            log::info!("New WebSocket connection ({path}): {addr}. There are {} webview and {} editor connections.", peers.webview_map.lock().await.len() + 1, peers.editor_map.lock().await.len());
            handle_webview_ws(addr, ws_stream, peers.webview_map.clone(), peers, state).await
        }
        "/editor" => {
            log::info!("New WebSocket connection ({path}): {addr}. There are {} webview and {} editor connections.", peers.webview_map.lock().await.len(), peers.editor_map.lock().await.len() + 1);
            handle_editor_ws(addr, ws_stream, peers.editor_map.clone()).await
        }
        _ => {}
    }
}

async fn handle_webview_ws(
    addr: SocketAddr,
    stream: WebSocketStream<TcpStream>,
    peers: PeerMap,
    peer_maps: PeerMaps,
    state: Arc<Mutex<crate::State>>,
) {
    let (write, read) = stream.split();

    let (tx, rx) = unbounded();

    tx.unbounded_send(tungstenite::Message::Binary(
        state.lock().await.current_content_payload.clone(),
    ))
    .unwrap();

    tx.unbounded_send(tungstenite::Message::Binary(
        state.lock().await.current_css_payload.clone(),
    ))
    .unwrap();

    tx.unbounded_send(tungstenite::Message::Binary(
        state.lock().await.current_frontmatter_payload.clone(),
    ))
    .unwrap();

    tx.unbounded_send(tungstenite::Message::Binary(
        state.lock().await.current_filename_payload.clone(),
    ))
    .unwrap();

    peers.lock().await.insert(addr, tx);

    let broadcast_incoming =
        read.try_for_each(|msg| webview::handle_incoming(msg, &peer_maps, state.clone()));
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

pub async fn send_to_all_editors(message: Vec<u8>, peers: EditorMap) {
    let sessions = &peers.lock().await;
    let broadcast_recipients = sessions.iter().map(|(_, (_, ws_sink))| ws_sink);
    for recp in broadcast_recipients {
        recp.unbounded_send(Message::Binary(message.clone()))
            .unwrap();
    }
}

async fn handle_editor_ws(addr: SocketAddr, stream: WebSocketStream<TcpStream>, peers: EditorMap) {
    let (write, read) = stream.split();

    let (tx, rx) = unbounded();

    let id = Uuid::new_v4().to_string();

    let id_payload = serde_json::to_string(&ServerToEditorMessage::EditorId(id.clone())).unwrap();
    tx.clone()
        .unbounded_send(tungstenite::Message::Binary(id_payload.as_bytes().to_vec()))
        .unwrap();

    peers.lock().await.insert(id.clone(), (addr, tx));

    let broadcast_incoming = read.try_for_each(|_msg| ok(()));
    let receive_from_others = rx.map(Ok).forward(write);

    pin_mut!(broadcast_incoming, receive_from_others);
    select(broadcast_incoming, receive_from_others).await;

    peers.lock().await.remove(&id);
}
