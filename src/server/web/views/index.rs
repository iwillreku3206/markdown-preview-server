use askama::Template;
use askama_axum::IntoResponse;
use axum_macros::debug_handler;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexPage {}

#[debug_handler]
pub async fn index_handler() -> impl IntoResponse {
    IndexPage {}
}
