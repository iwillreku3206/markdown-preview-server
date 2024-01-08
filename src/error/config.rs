#[derive(Debug)]
pub struct NoConfigFileError {}

impl std::fmt::Display for NoConfigFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Config file not found")
    }
}

impl std::error::Error for NoConfigFileError {}
