use std::borrow::Cow;
use std::sync::Arc;
use std::time::Duration;

use futures::lock::Mutex;
use notify::{Config, Event, PollWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc::{channel, Receiver};

use crate::util::constants::magic_bytes::BYTES_CSS;
use crate::web::ws::send_to_all;
use crate::State;

pub fn open_user_css(path: String) -> String {
    std::fs::read_to_string(
        shellexpand::env(&path.clone())
            .unwrap_or(Cow::from(path.clone()))
            .to_string(),
    )
    .unwrap_or_else(|e| {
        log::warn!("Error opening CSS file [{}]: {}", path, e);
        "".to_string()
    })
}

pub async fn watch_user_css(path: String, state: Arc<Mutex<State>>) {
    tokio::spawn(async_watch(path, state));
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

async fn async_watch(path: String, state: Arc<Mutex<State>>) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    while let Some(_res) = rx.recv().await {
        let mut payload = BYTES_CSS.to_vec();
        let mut css = open_user_css(path.clone()).clone().as_bytes().to_vec();

        payload.append(&mut css);
        state.lock().await.set_css_payload(payload.clone());

        let sessions = &state.lock().await.sessions;
        let _ = send_to_all(payload, sessions.webview_map.clone());
    }

    Ok(())
}
