use futures_util::lock::Mutex;
use headless_chrome::{types::PrintToPdfOptions, Browser, LaunchOptions};
use std::sync::Arc;

use crate::{hooks::toc::toc, State};

const MM_TO_INCH_DIVISOR: f64 = 25.4;

pub async fn export_to_pdf(
    state: Arc<Mutex<State>>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    log::info!("Generating PDF");
    let unlocked_state = state.lock().await;
    let parser = &unlocked_state.parser;
    let template = unlocked_state.current_template.clone();
    let markdown = unlocked_state.current_document.clone();

    let (document, frontmatter) = parser.parse(&markdown);
    let html = toc(&template.get_document(&document, &frontmatter), &markdown);

    let browser = Browser::new(LaunchOptions {
        headless: false,
        ..Default::default()
    })?;
    let tab = browser.new_tab()?;
    let encoded_html = urlencoding::encode(&html);
    let url = format!("data:text/html;charset=utf-8,{}", encoded_html);
    tab.navigate_to(&url)?;

    let pdf = tab.print_to_pdf(Some(PrintToPdfOptions {
        landscape: Some(template.landscape()),
        margin_top: Some(template.metadata.print_options.page_margin_top_mm / MM_TO_INCH_DIVISOR),
        margin_bottom: Some(template.metadata.print_options.page_margin_bottom_mm / MM_TO_INCH_DIVISOR),
        margin_left: Some(template.metadata.print_options.page_margin_left_mm / MM_TO_INCH_DIVISOR),
        margin_right: Some(template.metadata.print_options.page_margin_right_mm / MM_TO_INCH_DIVISOR),
        paper_width: Some(template.metadata.print_options.paper_width_mm / MM_TO_INCH_DIVISOR),
        paper_height: Some(template.metadata.print_options.paper_height_mm / MM_TO_INCH_DIVISOR),
        print_background: Some(true),
        ..Default::default()
    }))?;

    Ok(pdf)
}
