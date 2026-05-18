use super::AssetId;
use std::{io, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AssetError {
    #[error("failed to read asset manifest at {path}")]
    ManifestRead { path: PathBuf, source: io::Error },

    #[error("failed to parse asset manifest at {path}")]
    ManifestParse {
        path: PathBuf,
        source: toml::de::Error,
    },

    #[error("asset `{id}` was not found in the manifest")]
    MissingAsset { id: AssetId },

    #[error("failed to read asset `{id}` at {path} : {source}")]
    AssetRead {
        id: AssetId,
        path: PathBuf,
        source: io::Error,
    },
}
