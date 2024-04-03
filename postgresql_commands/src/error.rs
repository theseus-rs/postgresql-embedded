/// PostgreSQL command result type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// PostgreSQL command errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error when a command fails
    #[error("Command error: stdout={stdout}; stderr={stderr}")]
    CommandError { stdout: String, stderr: String },
    /// Error when IO operations fail
    #[error(transparent)]
    IoError(anyhow::Error),
    /// Error when a command fails to execute before the timeout is reached
    #[error(transparent)]
    TimeoutError(anyhow::Error),
}

/// Convert [standard IO errors](std::io::Error) to a [embedded errors](Error::IoError)
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
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
    fn test_from_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::Other, "test");
        let error = Error::from(io_error);
        assert_eq!(error.to_string(), "test");
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
