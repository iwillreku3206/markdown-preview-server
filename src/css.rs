pub fn open_user_css(path: String) -> String {
    std::fs::read_to_string(path.clone()).unwrap_or_else(|e| {
        log::warn!("Error opening CSS file [{}]: {}", path, e);
        "".to_string()
    })
}
