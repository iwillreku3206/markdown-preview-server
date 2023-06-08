use std::sync::Arc;

use axum::{
    body::{self, Full},
    extract::{rejection::PathRejection, Path, State},
    http::{Response, StatusCode},
    response::IntoResponse,
};
use futures::lock::Mutex;

use super::AppState;

pub async fn image(
    image: Result<Path<String>, PathRejection>,
    State(state): State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    match image {
        Ok(image) => {
            let image_dir_enabled = &state
                .lock()
                .await
                .pre_state
                .lock()
                .await
                .config
                .image_dir_enabled
                .clone();

            let image_dir = &state
                .lock()
                .await
                .pre_state
                .lock()
                .await
                .config
                .image_dir
                .clone()
                .to_owned();

            if !image_dir_enabled {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(body::boxed(Full::from("Image folder is disabled")))
                    .unwrap();
            }

            if image.contains('/') || image.contains('\\') {
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(body::boxed(Full::from("Cannot enter different directory")))
                    .unwrap();
            }

            match std::fs::read(format!(
                "{}/{}",
                shellexpand::env(image_dir).unwrap_or_default(),
                image.as_str()
            )) {
                Ok(file) => Response::builder()
                    .status(StatusCode::OK)
                    .header(
                        "Content-Type",
                        mime_guess::from_path(image.as_str())
                            .first()
                            .unwrap()
                            .to_string(),
                    )
                    .body(body::boxed(Full::from(file)))
                    .unwrap(),
                Err(_) => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(body::boxed(Full::from("File not found")))
                    .unwrap(),
            }
        }
        Err(PathRejection::MissingPathParams(_)) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(body::boxed(Full::from("Missing image name")))
            .unwrap(),
        Err(PathRejection::FailedToDeserializePathParams(_)) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(body::boxed(Full::from("Failed to deserialize image name")))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(body::boxed(Full::from("Internal server error")))
            .unwrap(),
    }
}
