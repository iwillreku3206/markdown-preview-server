use self::frame::{editor::EditorFrame, server::ServerFrame};

pub mod frame;
pub mod parse_frame;
pub mod stdio;

pub trait EditorConnection: Send + Sync {
    fn listen(&self);
    fn send(&self, frame: EditorFrame);
    fn close(&self);
}
