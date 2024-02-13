use std::string::FromUtf8Error;

/// PostgreSQL embedded result type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Errors that can occur when using PostgreSQL embedded
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error when PostgreSQL archive operations fail
    #[error(transparent)]
    ArchiveError(postgresql_archive::Error),
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

/// Convert PostgreSQL [archive errors](postgresql_archive::Error) to an [embedded errors](Error::ArchiveError)
impl From<postgresql_archive::Error> for Error {
    fn from(error: postgresql_archive::Error) -> Self {
        Error::ArchiveError(error)
    }
}

/// Convert [standard IO errors](std::io::Error) to a [embedded errors](Error::IoError)
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error.into())
    }
}

/// Convert [utf8 errors](FromUtf8Error) to [embedded errors](Error::IoError)
impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::IoError(error.into())
    }
}

#[cfg(feature = "tokio")]
/// Convert [elapsed time errors](tokio::time::error::Elapsed) to [embedded errors](Error::TimeoutError)
impl From<tokio::time::error::Elapsed> for Error {
    fn from(error: tokio::time::error::Elapsed) -> Self {
        Error::TimeoutError(error.into())
    }
}

/// These are relatively low value tests; they are here to reduce the coverage gap and
/// ensure that the error conversions are working as expected.
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_archive_error() {
        let archive_error = postgresql_archive::Error::ReleaseNotFound("test".to_string());
        let error = Error::from(archive_error);
        assert_eq!(error.to_string(), "release not found for version [test]");
    }

    #[test]
    fn test_from_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::Other, "test");
        let error = Error::from(io_error);
        assert_eq!(error.to_string(), "test");
    }

    #[test]
    fn test_from_utf8_error() {
        let invalid_utf8: Vec<u8> = vec![0, 159, 146, 150];
        let result = String::from_utf8(invalid_utf8);
        assert!(result.is_err());
        if let Err(error) = result {
            let error = Error::from(error);
            assert_eq!(
                error.to_string(),
                "invalid utf-8 sequence of 1 bytes from index 1"
            );
        }
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn test_from_elapsed_error() {
        let result = tokio::time::timeout(std::time::Duration::from_nanos(1), async {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        })
        .await;
        assert!(result.is_err());
        if let Err(elapsed_error) = result {
            let error = Error::from(elapsed_error);
            assert_eq!(error.to_string(), "deadline has elapsed");
        }
    }
}
