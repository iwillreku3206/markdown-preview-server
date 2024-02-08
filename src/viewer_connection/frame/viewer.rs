/// Data frame to be sent to the viewer
pub enum ViewerFrame {
    Ping,
    Pong,
    SetText(String),
    SetDocumentTitle(String),
    SetFilePath(String),
    SetCursorPositon(u64, u64),
    Close,
}
