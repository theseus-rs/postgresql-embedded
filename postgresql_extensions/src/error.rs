use std::sync::PoisonError;

/// PostgreSQL extensions result type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// PostgreSQL extensions errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Archive error
    #[error(transparent)]
    ArchiveError(#[from] postgresql_archive::Error),
    /// Error when a command fails
    #[error(transparent)]
    CommandError(#[from] postgresql_commands::Error),
    /// Extension not found
    #[error("extension not found '{0}'")]
    ExtensionNotFound(String),
    /// Error when an IO operation fails
    #[error("{0}")]
    IoError(String),
    /// Poisoned lock
    #[error("poisoned lock '{0}'")]
    PoisonedLock(String),
    /// Error when a regex operation fails
    #[error(transparent)]
    RegexError(#[from] regex_lite::Error),
    /// Error when a deserialization or serialization operation fails
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    /// Unsupported namespace
    #[error("unsupported namespace '{0}'")]
    UnsupportedNamespace(String),
}

/// Converts a [`std::sync::PoisonError<T>`] into a [`ParseError`](Error::PoisonedLock)
impl<T> From<PoisonError<T>> for Error {
    fn from(value: PoisonError<T>) -> Self {
        Error::PoisonedLock(value.to_string())
    }
}
