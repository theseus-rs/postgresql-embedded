use thiserror::Error;

/// PostgreSQL archive result type
pub type Result<T, E = ArchiveError> = core::result::Result<T, E>;

/// PostgreSQL archive errors
#[derive(Debug, Error)]
pub enum ArchiveError {
    /// Asset not found
    #[error("asset [{0}] not found")]
    AssetNotFound(String),
    /// Asset hash not found
    #[error("asset hash not found for asset [{0}]")]
    AssetHashNotFound(String),
    /// Invalid version
    #[error("version [{0}] is invalid")]
    InvalidVersion(String),
    /// IO error
    #[error(transparent)]
    IoError(anyhow::Error),
    /// Parse error
    #[error(transparent)]
    ParseError(anyhow::Error),
    /// Release not found
    #[error("release not found for version [{0}]")]
    ReleaseNotFound(String),
    /// Unexpected error
    #[error("{0}")]
    Unexpected(String),
}

/// Converts a [`regex::Error`] into an [`ParseError`](ArchiveError::ParseError)
impl From<regex::Error> for ArchiveError {
    fn from(error: regex::Error) -> Self {
        ArchiveError::ParseError(error.into())
    }
}

/// Converts a [`reqwest::Error`] into an [`IoError`](ArchiveError::IoError)
impl From<reqwest::Error> for ArchiveError {
    fn from(error: reqwest::Error) -> Self {
        ArchiveError::IoError(error.into())
    }
}

/// Converts a [`std::io::Error`] into an [`IoError`](ArchiveError::IoError)
impl From<std::io::Error> for ArchiveError {
    fn from(error: std::io::Error) -> Self {
        ArchiveError::IoError(error.into())
    }
}

/// Converts a [`std::num::ParseIntError`] into an [`ParseError`](ArchiveError::ParseError)
impl From<std::num::ParseIntError> for ArchiveError {
    fn from(error: std::num::ParseIntError) -> Self {
        ArchiveError::ParseError(error.into())
    }
}

/// Converts a [`std::path::StripPrefixError`] into an [`ParseError`](ArchiveError::ParseError)
impl From<std::path::StripPrefixError> for ArchiveError {
    fn from(error: std::path::StripPrefixError) -> Self {
        ArchiveError::ParseError(error.into())
    }
}
