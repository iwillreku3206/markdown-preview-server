use crate::editor_connection::EditorConnection;

pub struct Editor {
    pub editor_name: String,
    pub content_frame_count: u64,
    pub connection: Box<dyn EditorConnection>,
}
