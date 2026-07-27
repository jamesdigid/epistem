use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EpistemError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml_ng::Error),

    #[error("missing capability manifest at {0}")]
    MissingManifest(PathBuf),

    #[error("invalid capability manifest at {path}: {reason}")]
    InvalidManifest { path: PathBuf, reason: String },

    #[error("dependency resolution failed: {0}")]
    Resolution(String),

    #[error("registry error: {0}")]
    Registry(String),

    #[error("search error: {0}")]
    Search(String),
}

pub type Result<T> = std::result::Result<T, EpistemError>;
