/// PostgreSQL extensions result type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// PostgreSQL extensions errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Archive error
    #[error(transparent)]
    ArchiveFound(#[from] postgresql_archive::Error),
    /// Error when a command fails
    #[error(transparent)]
    CommandError(#[from] postgresql_commands::Error),
    /// Error when an IO operation fails
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// Poisoned lock
    #[error("poisoned lock '{0}'")]
    PoisonedLock(String),
    /// Error when a regex operation fails
    #[error(transparent)]
    RegexError(#[from] regex::Error),
    /// Error when a deserialization or serialization operation fails
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    /// Unsupported namespace
    #[error("unsupported namespace '{0}'")]
    UnsupportedNamespace(String),
}
