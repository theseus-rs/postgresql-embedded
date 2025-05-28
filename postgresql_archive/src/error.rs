/// PostgreSQL archive result type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// PostgreSQL archive errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Asset not found
    #[error("asset not found")]
    AssetNotFound,
    /// Asset hash not found
    #[error("asset hash not found for asset '{0}'")]
    AssetHashNotFound(String),
    /// Error when the hash of the archive does not match the expected hash
    #[error("Archive hash [{archive_hash}] does not match expected hash [{hash}]")]
    ArchiveHashMismatch { archive_hash: String, hash: String },
    /// Invalid version
    #[error("version '{0}' is invalid")]
    InvalidVersion(String),
    /// IO error
    #[error("{0}")]
    IoError(String),
    /// Parse error
    #[error("{0}")]
    ParseError(String),
    /// Poisoned lock
    #[error("poisoned lock '{0}'")]
    PoisonedLock(String),
    /// Repository failure
    #[error("{0}")]
    RepositoryFailure(String),
    /// Unexpected error
    #[error("{0}")]
    Unexpected(String),
    /// Unsupported extractor
    #[error("unsupported extractor for '{0}'")]
    UnsupportedExtractor(String),
    /// Unsupported hasher
    #[error("unsupported hasher for '{0}'")]
    UnsupportedHasher(String),
    /// Unsupported hasher
    #[error("unsupported matcher for '{0}'")]
    UnsupportedMatcher(String),
    /// Unsupported repository
    #[error("unsupported repository for '{0}'")]
    UnsupportedRepository(String),
    /// Version not found
    #[error("version not found for '{0}'")]
    VersionNotFound(String),
}

/// Converts a [`regex_lite::Error`] into an [`ParseError`](Error::ParseError)
impl From<regex_lite::Error> for Error {
    fn from(error: regex_lite::Error) -> Self {
        Error::ParseError(error.to_string())
    }
}

/// Converts a [`reqwest::Error`] into an [`IoError`](Error::IoError)
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::IoError(error.to_string())
    }
}

/// Converts a [`reqwest_middleware::Error`] into an [`IoError`](Error::IoError)
impl From<reqwest_middleware::Error> for Error {
    fn from(error: reqwest_middleware::Error) -> Self {
        Error::IoError(error.to_string())
    }
}

/// Converts a [`std::io::Error`] into an [`IoError`](Error::IoError)
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error.to_string())
    }
}

/// Converts a [`std::time::SystemTimeError`] into an [`IoError`](Error::IoError)
impl From<std::time::SystemTimeError> for Error {
    fn from(error: std::time::SystemTimeError) -> Self {
        Error::IoError(error.to_string())
    }
}

/// Converts a [`std::num::ParseIntError`] into an [`ParseError`](Error::ParseError)
impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::ParseError(error.to_string())
    }
}

/// Converts a [`semver::Error`] into an [`ParseError`](Error::ParseError)
impl From<semver::Error> for Error {
    fn from(error: semver::Error) -> Self {
        Error::IoError(error.to_string())
    }
}

/// Converts a [`std::path::StripPrefixError`] into an [`ParseError`](Error::ParseError)
impl From<std::path::StripPrefixError> for Error {
    fn from(error: std::path::StripPrefixError) -> Self {
        Error::ParseError(error.to_string())
    }
}

/// Converts a [`url::ParseError`] into an [`ParseError`](Error::ParseError)
impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Error::ParseError(error.to_string())
    }
}

/// These are relatively low value tests; they are here to reduce the coverage gap and
/// ensure that the error conversions are working as expected.
#[cfg(test)]
mod test {
    use super::*;
    use anyhow::anyhow;
    use semver::VersionReq;
    use std::ops::Add;
    use std::path::PathBuf;
    use std::str::FromStr;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_from_regex_error() {
        let regex_error = regex_lite::Regex::new("(?=a)").expect_err("regex error");
        let error = Error::from(regex_error);
        assert_eq!(error.to_string(), "look-around is not supported");
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
        let parse_int_error = u64::from_str("test").expect_err("parse int error");
        let error = Error::from(parse_int_error);
        assert_eq!(error.to_string(), "invalid digit found in string");
    }

    #[test]
    fn test_from_semver_error() {
        let semver_error = VersionReq::parse("foo").expect_err("semver error");
        let error = Error::from(semver_error);
        assert_eq!(
            error.to_string(),
            "unexpected character 'f' while parsing major version number"
        );
    }

    #[test]
    fn test_from_strip_prefix_error() {
        let path = PathBuf::from("test");
        let strip_prefix_error = path.strip_prefix("foo").expect_err("strip prefix error");
        let error = Error::from(strip_prefix_error);
        assert_eq!(error.to_string(), "prefix not found");
    }

    #[test]
    fn test_from_system_time_error() {
        let future_time = SystemTime::now().add(Duration::from_secs(300));
        let system_time_error = SystemTime::now()
            .duration_since(future_time)
            .expect_err("system time error");
        let error = Error::from(system_time_error);
        assert_eq!(
            error.to_string(),
            "second time provided was later than self"
        );
    }

    #[test]
    fn test_from_url_parse_error() {
        let parse_error = url::ParseError::EmptyHost;
        let error = Error::from(parse_error);
        assert_eq!(error.to_string(), "empty host");
    }
}
