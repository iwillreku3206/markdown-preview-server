use super::frame::server::ViewerServerFrame;

pub fn parse_frame(frame: &[u8]) -> Option<ViewerServerFrame> {
    if frame.len() < 2 {
        return None;
    }
    let frame_type: u16 = ((frame[0] as u16) << 8) | (frame[1] as u16);

    match frame_type {
        0x0001 => Some(ViewerServerFrame::Ping),
        0x0002 => Some(ViewerServerFrame::Pong),
        0xffff => Some(ViewerServerFrame::Close),
        _ => None,
    }
}
