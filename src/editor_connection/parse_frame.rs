use crate::util::ipc_replace_special_chars::ipc_replace_special_chars;

use super::frame::server::EditorServerFrame;

pub fn parse_frame(frame: &[u8]) -> Option<EditorServerFrame> {
    if frame.len() < 2 {
        return None;
    }

    let frame_type: u16 = ((frame[0] as u16) << 8) | (frame[1] as u16);
    match frame_type {
        0x0001 => Some(EditorServerFrame::Ping),
        0x0002 => Some(EditorServerFrame::Pong),
        0x0100 => {
            let str = ipc_replace_special_chars(&frame[2..]);

            Some(EditorServerFrame::SetText(str))
        }
        0x0101 => {
            let str = ipc_replace_special_chars(&frame[2..]);

            Some(EditorServerFrame::SetDocumentTitle(str))
        }
        0x0102 => {
            let str = ipc_replace_special_chars(&frame[2..]);

            Some(EditorServerFrame::SetFilePath(str))
        }
        0xffff => Some(EditorServerFrame::Close),
        _ => None,
    }
}
