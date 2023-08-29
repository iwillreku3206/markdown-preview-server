use axum::{body::Full, extract::State, http::Response, response::IntoResponse};
use futures_util::lock::Mutex;
use std::sync::Arc;

use crate::export::export_to_pdf;

pub async fn pdf(State(state): State<Arc<Mutex<crate::State>>>) -> impl IntoResponse {
    log::info!("PDF Requested");

    let generated = export_to_pdf(state.clone()).await;

    match generated {
        Ok(payload) => Response::builder()
            .status(200)
            .header("Content-Type", "application/pdf")
            .header("Content-Disposition", "attachment; filename=export.pdf")
            .body(Full::from(payload))
            .unwrap(),
        Err(_) => Response::builder()
            .status(500)
            .body(Full::from("Error generating PDF"))
            .unwrap(),
    }
}
