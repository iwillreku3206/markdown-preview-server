pub mod editor;
pub mod server;

pub trait Frame {
    fn to_string(&self) -> String;
}
