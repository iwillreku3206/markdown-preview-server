use axum::{body::Body, extract, http::Response};
use axum_macros::debug_handler;
use include_dir::{include_dir, Dir};

static STATIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/static");
static ERROR_404: &[u8] = &[
    b'4', b'0', b'4', b' ', b'N', b'o', b't', b' ', b'F', b'o', b'u', b'n', b'd', b'!',
];

#[debug_handler]
pub async fn static_dir_handler(extract::Path(path): extract::Path<String>) -> Response<Body> {
    let path = path.trim_start_matches('/');
    let file = STATIC_DIR.get_file(path);
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    if let Some(file) = file {
        Response::builder()
            .status(200)
            .header("Content-Type", mime.to_string())
            .body(Body::from(file.contents()))
            .unwrap()
    } else {
        Response::builder()
            .status(404)
            .header("Content-Type", "text/plain")
            .body(Body::from(ERROR_404))
            .unwrap()
    }
}
