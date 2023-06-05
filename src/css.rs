use std::sync::Arc;
use std::time::Duration;

use futures::lock::Mutex;
use notify::{Config, Event, PollWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc::{channel, Receiver};
use tungstenite::Message;

use crate::util::constants::magic_bytes::BYTES_CSS;
use crate::{PeerMap, PreState};

pub fn open_user_css(path: String) -> String {
    std::fs::read_to_string(path.clone()).unwrap_or_else(|e| {
        log::warn!("Error opening CSS file [{}]: {}", path, e);
        "".to_string()
    })
}

pub async fn watch_user_css(path: String, state: Arc<Mutex<PreState>>, sessions: PeerMap) {
    tokio::spawn(async_watch(path, state, sessions));
}

fn async_watcher() -> notify::Result<(PollWatcher, Receiver<notify::Result<Event>>)> {
    let (tx, rx) = channel(1);

    let watcher = PollWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::with_poll_interval(Config::default(), Duration::from_millis(250)),
    )?;

    Ok((watcher, rx))
}

async fn async_watch(
    path: String,
    state: Arc<Mutex<PreState>>,
    sessions: PeerMap,
) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    while let Some(_res) = rx.recv().await {
        let mut payload = BYTES_CSS.to_vec();
        let mut css = open_user_css(path.clone()).clone().as_bytes().to_vec();

        payload.append(&mut css);
        state.lock().await.set_css_payload(payload.clone());

        let sessions = &sessions.lock().await;
        let broadcast_recipients = sessions.iter().map(|(_, ws_sink)| ws_sink);
        for recp in broadcast_recipients {
            recp.unbounded_send(Message::Binary(payload.clone()))
                .unwrap()
        }
    }

    Ok(())
}
