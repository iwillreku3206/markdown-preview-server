use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum ServerToEditorMessage {
	EditorId(String),
    GotoPath(String),
}
