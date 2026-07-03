use std::{error, fmt, string::FromUtf8Error};

/// `PostgreSQL` embedded result type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Errors that can occur when using `PostgreSQL` embedded
#[derive(Debug)]
pub enum Error {
    /// Error when `PostgreSQL` archive operations fail
    ArchiveError(postgresql_archive::Error),
    /// Error when a command fails
    CommandError { stdout: String, stderr: String },
    /// Error when the database could not be created
    CreateDatabaseError(String),
    /// Error when accessing the database
    DatabaseError(sqlx::Error),
    /// Error when determining if the database exists
    DatabaseExistsError(String),
    /// Error when the database could not be initialized
    DatabaseInitializationError(String),
    /// Error when the database could not be started
    DatabaseStartError(String),
    /// Error when the database could not be stopped
    DatabaseStopError(String),
    /// Error when the database could not be dropped
    DropDatabaseError(String),
    /// Error when an invalid URL is provided
    InvalidUrl { url: String, message: String },
    /// Error when IO operations fail
    IoError(String),
    /// Parse error
    ParseError(semver::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArchiveError(error) => fmt::Display::fmt(error, formatter),
            Self::CommandError { stdout, stderr } => {
                write!(formatter, "Command error: stdout={stdout}; stderr={stderr}")
            }
            Self::CreateDatabaseError(error) => formatter.write_str(error),
            Self::DatabaseExistsError(error) => formatter.write_str(error),
            Self::DatabaseInitializationError(error) => formatter.write_str(error),
            Self::DatabaseStartError(error) => formatter.write_str(error),
            Self::DatabaseStopError(error) => formatter.write_str(error),
            Self::DropDatabaseError(error) => formatter.write_str(error),
            Self::IoError(error) => formatter.write_str(error),
            Self::DatabaseError(error) => fmt::Display::fmt(error, formatter),
            Self::InvalidUrl { url, message } => {
                write!(formatter, "Invalid URL: {url}; {message}")
            }
            Self::ParseError(error) => fmt::Display::fmt(error, formatter),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::ArchiveError(error) => error::Error::source(error),
            Self::DatabaseError(error) => error::Error::source(error),
            Self::ParseError(error) => error::Error::source(error),
            Self::CommandError { .. }
            | Self::CreateDatabaseError(_)
            | Self::DatabaseExistsError(_)
            | Self::DatabaseInitializationError(_)
            | Self::DatabaseStartError(_)
            | Self::DatabaseStopError(_)
            | Self::DropDatabaseError(_)
            | Self::InvalidUrl { .. }
            | Self::IoError(_) => None,
        }
    }
}

/// Convert `PostgreSQL` [archive errors](postgresql_archive::Error) to an [embedded errors](Error::ArchiveError)
impl From<postgresql_archive::Error> for Error {
    fn from(error: postgresql_archive::Error) -> Self {
        Error::ArchiveError(error)
    }
}

/// Convert [database errors](sqlx::Error) to [embedded errors](Error::DatabaseError)
impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Error::DatabaseError(error)
    }
}

/// Convert [version errors](semver::Error) to [embedded errors](Error::ParseError)
impl From<semver::Error> for Error {
    fn from(error: semver::Error) -> Self {
        Error::ParseError(error)
    }
}

/// Convert [standard IO errors](std::io::Error) to a [embedded errors](Error::IoError)
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error.to_string())
    }
}

/// Convert [utf8 errors](FromUtf8Error) to [embedded errors](Error::IoError)
impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::IoError(error.to_string())
    }
}

/// These are relatively low value tests; they are here to reduce the coverage gap and
/// ensure that the error conversions are working as expected.
#[cfg(test)]
mod test {
    use super::*;

    fn assert_display_and_no_source(error: Error, expected: &str) {
        assert_eq!(error.to_string(), expected);
        assert!(std::error::Error::source(&error).is_none());
    }

    fn assert_display_and_source(error: Error, expected: &str, source_message: &str) {
        assert_eq!(error.to_string(), expected);
        let source = std::error::Error::source(&error).expect("source error");
        assert_eq!(source.to_string(), source_message);
    }

    #[test]
    fn test_display_messages_and_sources() {
        assert_display_and_no_source(
            Error::ArchiveError(postgresql_archive::Error::VersionNotFound(
                "17.0".to_string(),
            )),
            "version not found for '17.0'",
        );
        assert_display_and_no_source(
            Error::CommandError {
                stdout: "createdb output".to_string(),
                stderr: "createdb error".to_string(),
            },
            "Command error: stdout=createdb output; stderr=createdb error",
        );
        assert_display_and_no_source(
            Error::CreateDatabaseError("create failed".to_string()),
            "create failed",
        );
        assert_display_and_source(
            Error::DatabaseError(sqlx::Error::Configuration(Box::new(std::io::Error::other(
                "configuration failed",
            )))),
            "error with configuration: configuration failed",
            "configuration failed",
        );
        assert_display_and_no_source(
            Error::DatabaseExistsError("exists failed".to_string()),
            "exists failed",
        );
        assert_display_and_no_source(
            Error::DatabaseInitializationError("init failed".to_string()),
            "init failed",
        );
        assert_display_and_no_source(
            Error::DatabaseStartError("start failed".to_string()),
            "start failed",
        );
        assert_display_and_no_source(
            Error::DatabaseStopError("stop failed".to_string()),
            "stop failed",
        );
        assert_display_and_no_source(
            Error::DropDatabaseError("drop failed".to_string()),
            "drop failed",
        );
        assert_display_and_no_source(
            Error::InvalidUrl {
                url: "postgres://localhost".to_string(),
                message: "missing database".to_string(),
            },
            "Invalid URL: postgres://localhost; missing database",
        );
        assert_display_and_no_source(Error::IoError("io failure".to_string()), "io failure");
        assert_display_and_no_source(
            Error::ParseError(semver::VersionReq::parse("latest").expect_err("semver error")),
            "unexpected character 'l' while parsing major version number",
        );
    }

    #[test]
    fn test_from_archive_error() {
        let archive_error = postgresql_archive::Error::VersionNotFound("test".to_string());
        let error = Error::from(archive_error);
        assert_eq!(error.to_string(), "version not found for 'test'");
    }

    #[test]
    fn test_from_sqlx_error() {
        let sqlx_error = sqlx::Error::InvalidArgument("invalid database argument".to_string());
        let error = Error::from(sqlx_error);
        assert_eq!(error.to_string(), "invalid database argument");
    }

    #[test]
    fn test_from_semver_error() {
        let semver_error = semver::VersionReq::parse("latest").expect_err("semver error");
        let error = Error::from(semver_error);
        assert_eq!(
            error.to_string(),
            "unexpected character 'l' while parsing major version number"
        );
    }

    #[test]
    fn test_from_io_error() {
        let io_error = std::io::Error::other("test");
        let error = Error::from(io_error);
        assert_eq!(error.to_string(), "test");
    }

    #[test]
    fn test_from_utf8_error() {
        let invalid_utf8: Vec<u8> = vec![0, 159, 146, 150];
        let from_utf8_error = String::from_utf8(invalid_utf8).expect_err("from utf8 error");
        let error = Error::from(from_utf8_error);
        assert_eq!(
            error.to_string(),
            "invalid utf-8 sequence of 1 bytes from index 1"
        );
    }
}
