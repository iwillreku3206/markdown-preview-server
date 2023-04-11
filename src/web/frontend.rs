use axum::{
    body::{self, Full},
    extract::Path,
    http::{header, HeaderValue, Response, StatusCode, Uri},
    response::IntoResponse,
};
use include_dir::include_dir;
use include_dir::Dir;

static WEB_BUILD: Dir<'_> = include_dir!("assets/web_build");
pub async fn frontend(_path: Uri) -> impl IntoResponse {
    let path = _path.to_string().trim_start_matches('/').to_string();
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
}
