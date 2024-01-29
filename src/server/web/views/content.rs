use askama::Template;
use askama_axum::IntoResponse;
use axum_macros::debug_handler;

#[derive(Template)]
#[template(path = "content.html")]
pub struct ContentPage {}

#[debug_handler]
pub async fn content_handler() -> impl IntoResponse {
    ContentPage {}
}
