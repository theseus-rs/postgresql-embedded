use std::{error, fmt, sync::PoisonError};

/// PostgreSQL extensions result type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// PostgreSQL extensions errors
#[derive(Debug)]
pub enum Error {
    /// Archive error
    ArchiveError(postgresql_archive::Error),
    /// Error when a command fails
    CommandError(postgresql_commands::Error),
    /// Extension not found
    ExtensionNotFound(String),
    /// Error when an IO operation fails
    IoError(String),
    /// Poisoned lock
    PoisonedLock(String),
    /// Error when a regex operation fails
    RegexError(regex_lite::Error),
    /// Error when a deserialization or serialization operation fails
    SerdeError(serde_json::Error),
    /// Unsupported namespace
    UnsupportedNamespace(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArchiveError(error) => fmt::Display::fmt(error, formatter),
            Self::CommandError(error) => fmt::Display::fmt(error, formatter),
            Self::ExtensionNotFound(extension) => {
                write!(formatter, "extension not found '{extension}'")
            }
            Self::IoError(error) => formatter.write_str(error),
            Self::PoisonedLock(lock) => write!(formatter, "poisoned lock '{lock}'"),
            Self::RegexError(error) => fmt::Display::fmt(error, formatter),
            Self::SerdeError(error) => fmt::Display::fmt(error, formatter),
            Self::UnsupportedNamespace(namespace) => {
                write!(formatter, "unsupported namespace '{namespace}'")
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::ArchiveError(error) => error::Error::source(error),
            Self::CommandError(error) => error::Error::source(error),
            Self::RegexError(error) => error::Error::source(error),
            Self::SerdeError(error) => error::Error::source(error),
            Self::ExtensionNotFound(_)
            | Self::IoError(_)
            | Self::PoisonedLock(_)
            | Self::UnsupportedNamespace(_) => None,
        }
    }
}

/// Converts a [`postgresql_archive::Error`] into an [`ArchiveError`](Error::ArchiveError)
impl From<postgresql_archive::Error> for Error {
    fn from(error: postgresql_archive::Error) -> Self {
        Error::ArchiveError(error)
    }
}

/// Converts a [`postgresql_commands::Error`] into a [`CommandError`](Error::CommandError)
impl From<postgresql_commands::Error> for Error {
    fn from(error: postgresql_commands::Error) -> Self {
        Error::CommandError(error)
    }
}

/// Converts a [`regex_lite::Error`] into a [`RegexError`](Error::RegexError)
impl From<regex_lite::Error> for Error {
    fn from(error: regex_lite::Error) -> Self {
        Error::RegexError(error)
    }
}

/// Converts a [`serde_json::Error`] into a [`SerdeError`](Error::SerdeError)
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeError(error)
    }
}

/// Converts a [`std::sync::PoisonError<T>`] into a [`ParseError`](Error::PoisonedLock)
impl<T> From<PoisonError<T>> for Error {
    fn from(value: PoisonError<T>) -> Self {
        Error::PoisonedLock(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_display_and_no_source(error: Error, expected: &str) {
        assert_eq!(error.to_string(), expected);
        assert!(std::error::Error::source(&error).is_none());
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
            Error::CommandError(postgresql_commands::Error::CommandError {
                stdout: "install output".to_string(),
                stderr: "install error".to_string(),
            }),
            "Command error: stdout=install output; stderr=install error",
        );
        assert_display_and_no_source(
            Error::ExtensionNotFound("pg_stat_statements".to_string()),
            "extension not found 'pg_stat_statements'",
        );
        assert_display_and_no_source(Error::IoError("io failure".to_string()), "io failure");
        assert_display_and_no_source(
            Error::PoisonedLock("extensions".to_string()),
            "poisoned lock 'extensions'",
        );
        assert_display_and_no_source(
            Error::RegexError(regex_lite::Regex::new("(?=a)").expect_err("regex error")),
            "look-around is not supported",
        );
        assert_display_and_no_source(
            Error::SerdeError(serde_json::Error::io(std::io::Error::other(
                "json io failure",
            ))),
            "json io failure",
        );
        assert_display_and_no_source(
            Error::UnsupportedNamespace("unknown".to_string()),
            "unsupported namespace 'unknown'",
        );
    }

    #[test]
    fn test_from_archive_error() {
        let archive_error = postgresql_archive::Error::VersionNotFound("17.0".to_string());
        let error = Error::from(archive_error);
        assert_eq!(error.to_string(), "version not found for '17.0'");
    }

    #[test]
    fn test_from_command_error() {
        let command_error = postgresql_commands::Error::IoError("command io".to_string());
        let error = Error::from(command_error);
        assert_eq!(error.to_string(), "command io");
    }

    #[test]
    fn test_from_regex_error() {
        let regex_error = regex_lite::Regex::new("(?=a)").expect_err("regex error");
        let error = Error::from(regex_error);
        assert_eq!(error.to_string(), "look-around is not supported");
    }

    #[test]
    fn test_from_serde_error() {
        let serde_error = serde_json::from_str::<serde_json::Value>("{").expect_err("serde error");
        let error = Error::from(serde_error);
        assert_eq!(
            error.to_string(),
            "EOF while parsing an object at line 1 column 1"
        );
    }

    #[test]
    fn test_from_poison_error() {
        let error = Error::from(std::sync::PoisonError::new(()));
        assert!(error.to_string().contains("poisoned lock"));
    }
}
