use super::frame::server::ServerFrame;

pub fn parse_frame(frame: &[u8]) -> Option<ServerFrame> {
    if frame.len() < 2 {
        return None;
    }

    let frame_type: u16 = ((frame[0] as u16) << 8) | (frame[1] as u16);
    match frame_type {
        0x0001 => Some(ServerFrame::Ping),
        0x0002 => Some(ServerFrame::Pong),
        0x0100 => {
            let str = String::from_utf8(frame[2..].to_vec()).ok();

            match str {
                Some(s) => Some(ServerFrame::SetText(s)),
                None => None,
            }
        }
        0x0101 => {
            let str = String::from_utf8(frame[2..].to_vec()).ok();

            match str {
                Some(s) => Some(ServerFrame::SetDocumentTitle(s)),
                None => None,
            }
        }
        0x0102 => {
            let str = String::from_utf8(frame[2..].to_vec()).ok();

            match str {
                Some(s) => Some(ServerFrame::SetFilePath(s)),
                None => None,
            }
        }
        0xffff => Some(ServerFrame::Close),
        _ => None,
    }
}
