use std::{error, fmt, sync::PoisonError};

/// PostgreSQL archive result type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// PostgreSQL archive errors
#[derive(Debug)]
pub enum Error {
    /// Asset not found
    AssetNotFound,
    /// Asset hash not found
    AssetHashNotFound(String),
    /// Error when the hash of the archive does not match the expected hash
    ArchiveHashMismatch { archive_hash: String, hash: String },
    /// Invalid version
    InvalidVersion(String),
    /// IO error
    IoError(String),
    /// Parse error
    ParseError(String),
    /// Poisoned lock
    PoisonedLock(String),
    /// Repository failure
    RepositoryFailure(String),
    /// Unexpected error
    Unexpected(String),
    /// Unsupported extractor
    UnsupportedExtractor(String),
    /// Unsupported hasher
    UnsupportedHasher(String),
    /// Unsupported hasher
    UnsupportedMatcher(String),
    /// Unsupported repository
    UnsupportedRepository(String),
    /// Version not found
    VersionNotFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AssetNotFound => formatter.write_str("asset not found"),
            Self::AssetHashNotFound(asset) => {
                write!(formatter, "asset hash not found for asset '{asset}'")
            }
            Self::ArchiveHashMismatch { archive_hash, hash } => write!(
                formatter,
                "Archive hash [{archive_hash}] does not match expected hash [{hash}]"
            ),
            Self::InvalidVersion(version) => write!(formatter, "version '{version}' is invalid"),
            Self::IoError(error) => formatter.write_str(error),
            Self::ParseError(error) => formatter.write_str(error),
            Self::RepositoryFailure(error) => formatter.write_str(error),
            Self::Unexpected(error) => formatter.write_str(error),
            Self::PoisonedLock(lock) => write!(formatter, "poisoned lock '{lock}'"),
            Self::UnsupportedExtractor(extractor) => {
                write!(formatter, "unsupported extractor for '{extractor}'")
            }
            Self::UnsupportedHasher(hasher) => {
                write!(formatter, "unsupported hasher for '{hasher}'")
            }
            Self::UnsupportedMatcher(matcher) => {
                write!(formatter, "unsupported matcher for '{matcher}'")
            }
            Self::UnsupportedRepository(repository) => {
                write!(formatter, "unsupported repository for '{repository}'")
            }
            Self::VersionNotFound(version) => {
                write!(formatter, "version not found for '{version}'")
            }
        }
    }
}

impl error::Error for Error {}

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

#[cfg(feature = "maven")]
/// Converts a [`quick_xml::DeError`] into a [`ParseError`](Error::ParseError)
impl From<quick_xml::DeError> for Error {
    fn from(error: quick_xml::DeError) -> Self {
        Error::ParseError(error.to_string())
    }
}

#[cfg(feature = "zip")]
/// Converts a [`zip::result::ZipError`] into a [`ParseError`](Error::Unexpected)
impl From<zip::result::ZipError> for Error {
    fn from(error: zip::result::ZipError) -> Self {
        Error::Unexpected(error.to_string())
    }
}

/// Converts a [`std::sync::PoisonError<T>`] into a [`ParseError`](Error::PoisonedLock)
impl<T> From<PoisonError<T>> for Error {
    fn from(value: PoisonError<T>) -> Self {
        Error::PoisonedLock(value.to_string())
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

    fn assert_display(error: Error, expected: &str) {
        assert_eq!(error.to_string(), expected);
        assert!(std::error::Error::source(&error).is_none());
    }

    #[test]
    fn test_display_messages() {
        assert_display(Error::AssetNotFound, "asset not found");
        assert_display(
            Error::AssetHashNotFound("postgres.tar.gz".to_string()),
            "asset hash not found for asset 'postgres.tar.gz'",
        );
        assert_display(
            Error::ArchiveHashMismatch {
                archive_hash: "actual".to_string(),
                hash: "expected".to_string(),
            },
            "Archive hash [actual] does not match expected hash [expected]",
        );
        assert_display(
            Error::InvalidVersion("latest".to_string()),
            "version 'latest' is invalid",
        );
        assert_display(Error::IoError("io failure".to_string()), "io failure");
        assert_display(
            Error::ParseError("parse failure".to_string()),
            "parse failure",
        );
        assert_display(
            Error::PoisonedLock("settings".to_string()),
            "poisoned lock 'settings'",
        );
        assert_display(
            Error::RepositoryFailure("repository failure".to_string()),
            "repository failure",
        );
        assert_display(Error::Unexpected("unexpected".to_string()), "unexpected");
        assert_display(
            Error::UnsupportedExtractor("rar".to_string()),
            "unsupported extractor for 'rar'",
        );
        assert_display(
            Error::UnsupportedHasher("crc32".to_string()),
            "unsupported hasher for 'crc32'",
        );
        assert_display(
            Error::UnsupportedMatcher("custom".to_string()),
            "unsupported matcher for 'custom'",
        );
        assert_display(
            Error::UnsupportedRepository("mirror".to_string()),
            "unsupported repository for 'mirror'",
        );
        assert_display(
            Error::VersionNotFound("17.0".to_string()),
            "version not found for '17.0'",
        );
    }

    #[test]
    fn test_from_regex_error() {
        let regex_error = regex_lite::Regex::new("(?=a)").expect_err("regex error");
        let error = Error::from(regex_error);
        assert_eq!(error.to_string(), "look-around is not supported");
    }

    #[test]
    fn test_from_reqwest_error() {
        let reqwest_error = reqwest::Client::new()
            .get("http://")
            .build()
            .expect_err("reqwest error");
        let error = Error::from(reqwest_error);
        assert!(error.to_string().contains("builder error"));
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

    #[cfg(feature = "maven")]
    #[test]
    fn test_from_quick_xml_error() {
        let xml = "<invalid>";
        let quick_xml_error = quick_xml::de::from_str::<String>(xml).expect_err("quick_xml error");
        let message = quick_xml_error.to_string();
        let error = Error::from(quick_xml_error);
        assert_eq!(error.to_string(), message);
    }

    #[cfg(feature = "zip")]
    #[test]
    fn test_from_zip_error() {
        let zip_error = zip::result::ZipError::FileNotFound;
        let message = zip_error.to_string();
        let error = Error::from(zip_error);
        assert_eq!(error.to_string(), message);
    }

    #[test]
    fn test_from_poisoned_lock() {
        let poison_error = std::sync::PoisonError::new(());
        let message = format!("poisoned lock '{poison_error}'");
        let error = Error::from(poison_error);
        assert_eq!(error.to_string(), message);
    }
}
