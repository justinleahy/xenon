#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum RuntimeError {
    #[error("window width must be greater than zero, got {width}")]
    InvalidWindowWidth { width: u32 },
    #[error("window height must be greater than zero, got {height}")]
    InvalidWindowHeight { height: u32 },
    #[error("invalid runtime config: {0}")]
    InvalidConfig(String),
}
