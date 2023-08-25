use std::sync::Arc;

use axum::{
    body::{self, Bytes, Full},
    extract::State,
    http::{header, HeaderValue, Response, StatusCode, Uri},
    response::IntoResponse,
};

use axum_macros::debug_handler;
use futures::lock::Mutex;
use include_dir::include_dir;
use include_dir::Dir;

static WEB_BUILD: Dir<'_> = include_dir!("assets/web_build");

const SVELTE_PATHS: [&str; 2] = ["", "content"];

#[debug_handler]
pub async fn frontend(
    _path: Uri,
    State(state): State<Arc<Mutex<crate::State>>>,
) -> impl IntoResponse {
    let unlocked_state = state.lock().await;

    let frontend_address = &unlocked_state.config.frontend_address;
    let mut path = _path.to_string().trim_start_matches('/').to_string();
    if SVELTE_PATHS.to_vec().contains(&path.as_str()) {
        path = "".to_string();
    }

    if frontend_address.is_empty() {
        if path.is_empty() {
            return Response::builder()
                .status(StatusCode::FOUND)
                .body(body::boxed(Full::from(
                    WEB_BUILD.get_file("index.html").unwrap().contents(),
                )))
                .unwrap();
        }
        let mime_type = mime_guess::from_path(&path).first_or_text_plain();

        match WEB_BUILD.get_file(&path) {
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body::boxed(Full::from("Not found")))
                .unwrap(),
            Some(file) => Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(mime_type.as_ref()).unwrap(),
                )
                .body(body::boxed(Full::from(file.contents())))
                .unwrap(),
        }
    } else {
        let uri = frontend_address.to_owned() + "/" + &path;
        let _res = reqwest::get(uri).await;
        let res = _res.unwrap();

        let status = res.status();
        let headers_list = res.headers().clone();

        let body = res.bytes().await.unwrap_or_else(|_| Bytes::new());

        let mut builder = Response::builder().status(status);
        let headers = builder.headers_mut().unwrap();

        for header in headers_list {
            headers.insert(header.0.unwrap(), header.1);
        }

        headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());

        builder
            .body(body::boxed(Full::from(body.to_vec())))
            .unwrap()
    }
}
