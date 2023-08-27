use axum::extract::State;
use futures_util::lock::Mutex;
use std::sync::Arc;

use crate::export::export_to_pdf;

pub async fn pdf(State(state): State<Arc<Mutex<crate::State>>>) -> Vec<u8> {
    log::info!("PDF Requested");

    let generated = export_to_pdf(state.clone()).await;

    generated.unwrap_or_default()
}
