use std::{error, fmt};

/// `PostgreSQL` command result type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// `PostgreSQL` command errors
#[derive(Debug)]
pub enum Error {
    /// Error when a command fails
    CommandError { stdout: String, stderr: String },
    /// Error when IO operations fail
    IoError(String),
    /// Error when a command fails to execute before the timeout is reached
    TimeoutError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandError { stdout, stderr } => {
                write!(formatter, "Command error: stdout={stdout}; stderr={stderr}")
            }
            Self::IoError(error) | Self::TimeoutError(error) => formatter.write_str(error),
        }
    }
}

impl error::Error for Error {}

/// Convert [standard IO errors](std::io::Error) to a [embedded errors](Error::IoError)
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error.to_string())
    }
}

#[cfg(feature = "tokio")]
/// Convert [elapsed time errors](tokio::time::error::Elapsed) to [embedded errors](Error::TimeoutError)
impl From<tokio::time::error::Elapsed> for Error {
    fn from(error: tokio::time::error::Elapsed) -> Self {
        Error::TimeoutError(error.to_string())
    }
}

/// These are relatively low value tests; they are here to reduce the coverage gap and
/// ensure that the error conversions are working as expected.
#[cfg(test)]
mod test {
    use super::*;

    fn assert_display(error: Error, expected: &str) {
        assert_eq!(error.to_string(), expected);
        assert!(std::error::Error::source(&error).is_none());
    }

    #[test]
    fn test_display_messages() {
        assert_display(
            Error::CommandError {
                stdout: "createdb output".to_string(),
                stderr: "createdb error".to_string(),
            },
            "Command error: stdout=createdb output; stderr=createdb error",
        );
        assert_display(Error::IoError("io failure".to_string()), "io failure");
        assert_display(
            Error::TimeoutError("deadline has elapsed".to_string()),
            "deadline has elapsed",
        );
    }

    #[test]
    fn test_from_io_error() {
        let io_error = std::io::Error::other("test");
        let error = Error::from(io_error);
        assert_eq!(error.to_string(), "test");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn test_from_elapsed_error() {
        let result = tokio::time::timeout(
            std::time::Duration::from_nanos(1),
            tokio::time::sleep(std::time::Duration::from_secs(1)),
        )
        .await;
        assert!(result.is_err());
        let elapsed_error = result.expect_err("elapsed error");
        let error = Error::from(elapsed_error);
        assert_eq!(error.to_string(), "deadline has elapsed");
    }
}
