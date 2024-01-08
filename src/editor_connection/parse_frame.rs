use super::frame::server::ServerFrame;

pub fn parse_frame(frame: &[u8]) -> ServerFrame {
    ServerFrame::Ping
}
