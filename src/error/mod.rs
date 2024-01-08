pub mod config;

pub type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug)]
pub struct FileNotFoundError {
    pub path: String,
}

impl std::fmt::Display for FileNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File not found: {}", self.path)
    }
}

impl std::error::Error for FileNotFoundError {}
