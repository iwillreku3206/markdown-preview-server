use axum::{
    body::{self, Bytes, Full},
    extract::State,
    http::{header, HeaderValue, Response, StatusCode, Uri},
    response::IntoResponse,
};
use include_dir::include_dir;
use include_dir::Dir;

use super::AppState;

static WEB_BUILD: Dir<'_> = include_dir!("assets/web_build");
pub async fn frontend(State(state): State<AppState>, _path: Uri) -> impl IntoResponse {
    let frontend_address = state.pre_state.args.frontend_address;
    let path = _path.to_string().trim_start_matches('/').to_string();
    if frontend_address.is_empty() {
        if path.is_empty() {
            return Response::builder()
                .status(StatusCode::FOUND)
                .header(header::LOCATION, "/index.html")
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
        let uri = frontend_address + "/" + &path;
        let _res = reqwest::get(uri).await;
        let res = _res.unwrap();

        let status = res.status();
        let body = res.bytes().await.unwrap_or_else(|_| Bytes::new());

        Response::builder()
            .status(status)
            .body(body::boxed(Full::from(body)))
            .unwrap()
    }
}
