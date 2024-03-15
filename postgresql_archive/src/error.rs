/// PostgreSQL archive result type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// PostgreSQL archive errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Asset not found
    #[error("asset [{0}] not found")]
    AssetNotFound(String),
    /// Asset hash not found
    #[error("asset hash not found for asset [{0}]")]
    AssetHashNotFound(String),
    /// Error when the hash of the archive does not match the expected hash
    #[error("Archive hash [{archive_hash}] does not match expected hash [{hash}]")]
    ArchiveHashMismatch { archive_hash: String, hash: String },
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

/// Converts a [`regex::Error`] into an [`ParseError`](Error::ParseError)
impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Error::ParseError(error.into())
    }
}

/// Converts a [`reqwest::Error`] into an [`IoError`](Error::IoError)
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::IoError(error.into())
    }
}

/// Converts a [`reqwest_middleware::Error`] into an [`IoError`](Error::IoError)
impl From<reqwest_middleware::Error> for Error {
    fn from(error: reqwest_middleware::Error) -> Self {
        Error::IoError(error.into())
    }
}

/// Converts a [`std::io::Error`] into an [`IoError`](Error::IoError)
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error.into())
    }
}

/// Converts a [`std::time::SystemTimeError`] into an [`IoError`](Error::IoError)
impl From<std::time::SystemTimeError> for Error {
    fn from(error: std::time::SystemTimeError) -> Self {
        Error::IoError(error.into())
    }
}

/// Converts a [`std::num::ParseIntError`] into an [`ParseError`](Error::ParseError)
impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::ParseError(error.into())
    }
}

/// Converts a [`std::path::StripPrefixError`] into an [`ParseError`](Error::ParseError)
impl From<std::path::StripPrefixError> for Error {
    fn from(error: std::path::StripPrefixError) -> Self {
        Error::ParseError(error.into())
    }
}

/// Converts a [`anyhow::Error`] into an [`Unexpected`](Error::Unexpected)
impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Error::Unexpected(error.to_string())
    }
}

/// These are relatively low value tests; they are here to reduce the coverage gap and
/// ensure that the error conversions are working as expected.
#[cfg(test)]
mod test {
    use super::*;
    use anyhow::anyhow;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_from_regex_error() {
        let regex_error = regex::Error::Syntax("test".to_string());
        let error = Error::from(regex_error);
        assert_eq!(error.to_string(), "test");
    }

    #[tokio::test]
    async fn test_from_reqwest_error() {
        let result = reqwest::get("https://a.com").await;
        assert!(result.is_err());
        if let Err(error) = result {
            let error = Error::from(error);
            assert!(error.to_string().contains("https://a.com"));
        }
    }

    #[tokio::test]
    async fn test_from_reqwest_middeleware_error() {
        let reqwest_middleware_error =
            reqwest_middleware::Error::Middleware(anyhow!("middleware error: test"));
        let error = Error::from(reqwest_middleware_error);
        assert!(error.to_string().contains("middleware error: test"));
    }

    #[test]
    fn test_from_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let error = Error::from(io_error);
        assert_eq!(error.to_string(), "test");
    }

    #[test]
    fn test_from_parse_int_error() {
        let result = u64::from_str("test");
        assert!(result.is_err());
        if let Err(error) = result {
            let error = Error::from(error);
            assert_eq!(error.to_string(), "invalid digit found in string");
        }
    }

    #[test]
    fn test_from_strip_prefix_error() {
        let path = PathBuf::from("test");
        let result = path.strip_prefix("foo");
        assert!(result.is_err());
        if let Err(error) = result {
            let error = Error::from(error);
            assert_eq!(error.to_string(), "prefix not found");
        }
    }

    #[test]
    fn test_from_anyhow_error() {
        let anyhow_error = anyhow::Error::msg("test");
        let error = Error::from(anyhow_error);
        assert_eq!(error.to_string(), "test");
    }
}
