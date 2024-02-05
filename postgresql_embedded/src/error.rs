use postgresql_archive::ArchiveError;
use std::string::FromUtf8Error;
use thiserror::Error;

/// PostgreSQL embedded result type
pub type Result<T, E = EmbeddedError> = core::result::Result<T, E>;

/// Errors that can occur when using PostgreSQL embedded
#[derive(Debug, Error)]
pub enum EmbeddedError {
    /// Error when PostgreSQL archive operations fail
    #[error(transparent)]
    ArchiveError(anyhow::Error),
    /// Error when the hash of the archive does not match the expected hash
    #[error("Archive hash [{archive_hash}] does not match expected hash [{hash}]")]
    ArchiveHashMismatch { archive_hash: String, hash: String },
    /// Error when the archive is not found for a specific version
    #[error("Archive not found for version [{0}]")]
    ArchiveNotFound(String),
    /// Error when a command fails
    #[error("Command error: stdout={stdout}; stderr={stderr}")]
    CommandError { stdout: String, stderr: String },
    /// Error when the database could not be created
    #[error(transparent)]
    CreateDatabaseError(anyhow::Error),
    /// Error when determining if the database exists
    #[error(transparent)]
    DatabaseExistsError(anyhow::Error),
    /// Error when the database could not be initialized
    #[error(transparent)]
    DatabaseInitializationError(anyhow::Error),
    /// Error when the database could not be started
    #[error(transparent)]
    DatabaseStartError(anyhow::Error),
    /// Error when the database could not be stopped
    #[error(transparent)]
    DatabaseStopError(anyhow::Error),
    /// Error when the database could not be dropped
    #[error(transparent)]
    DropDatabaseError(anyhow::Error),
    /// Error when IO operations fail
    #[error(transparent)]
    IoError(anyhow::Error),
    /// Error when a command fails to execute before the timeout is reached
    #[error(transparent)]
    TimeoutError(anyhow::Error),
}

/// Convert PostgreSQL [archive errors](ArchiveError) to an [embedded errors](EmbeddedError::ArchiveError)
impl From<ArchiveError> for EmbeddedError {
    fn from(error: ArchiveError) -> Self {
        EmbeddedError::ArchiveError(error.into())
    }
}

/// Convert [standard IO errors](std::io::Error) to a [embedded errors](EmbeddedError::IoError)
impl From<std::io::Error> for EmbeddedError {
    fn from(error: std::io::Error) -> Self {
        EmbeddedError::IoError(error.into())
    }
}

/// Convert [utf8 errors](FromUtf8Error) to [embedded errors](EmbeddedError::IoError)
impl From<FromUtf8Error> for EmbeddedError {
    fn from(error: FromUtf8Error) -> Self {
        EmbeddedError::IoError(error.into())
    }
}

#[cfg(feature = "tokio")]
/// Convert [elapsed time errors](tokio::time::error::Elapsed) to [embedded errors](EmbeddedError::TimeoutError)
impl From<tokio::time::error::Elapsed> for EmbeddedError {
    fn from(error: tokio::time::error::Elapsed) -> Self {
        EmbeddedError::TimeoutError(error.into())
    }
}
