use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("window width must be greater than zero, got {width}")]
    InvalidWindowWidth { width: u32 },

    #[error("window height must be greater than zero, got {height}")]
    InvalidWindowHeight { height: u32 },

    #[error("invalid runtime config: {0}")]
    InvalidConfig(String),

    #[error("failed to read config file {path}")]
    ConfigRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse config file {path}")]
    ConfigParse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
}
