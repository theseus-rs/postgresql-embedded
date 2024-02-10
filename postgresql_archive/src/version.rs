//! PostgreSQL version
#![allow(dead_code)]

use crate::error::ArchiveError::InvalidVersion;
use crate::error::{ArchiveError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// PostgreSQL version struct. The version is a simple wrapper around a string.
/// [Actively supported](https://www.postgresql.org/developer/roadmap/) major versions of
/// PostgreSQL are defined as constants. The oldest supported version is will be marked
/// as deprecated. Deprecated versions will be removed in a future release following semver
/// conventions for this crate.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Version {
    pub major: u64,
    pub minor: Option<u64>,
    pub release: Option<u64>,
}

/// The latest PostgreSQL version
pub const LATEST: Version = V16;

/// The latest PostgreSQL version 16
pub const V16: Version = Version::new(16, None, None);

/// The latest PostgreSQL version 15
pub const V15: Version = Version::new(15, None, None);

/// The latest PostgreSQL version 14
pub const V14: Version = Version::new(14, None, None);

/// The latest PostgreSQL version 13
pub const V13: Version = Version::new(13, None, None);

/// The latest PostgreSQL version 12
#[allow(deprecated)]
#[deprecated(
    since = "0.1.0",
    note = "See https://www.postgresql.org/developer/roadmap/"
)]
pub const V12: Version = Version::new(12, None, None);

impl Version {
    pub const fn new(major: u64, minor: Option<u64>, release: Option<u64>) -> Self {
        Self {
            major,
            minor,
            release,
        }
    }

    /// Matches the version against another version.  Provides a simple way to match
    /// against a major, major/minor, or major/minor/release version.  Returns `true`
    /// if the major version matches and the minor version matches or is not specified.
    /// Returns `true` if the major and minor versions match and the release matches or
    /// is not specified. Returns `false` otherwise.
    ///
    /// # Examples
    /// The methods of this trait must be consistent with each other and with those of [`PartialEq`].
    /// The following conditions must hold:
    ///
    /// 1. `16` matches `16.1.0`, `16.1.1`, `16.2.0`, etc.
    /// 2. `16.1` matches `16.1.0`, `16.1.1`, etc.
    /// 3. `16.1.0` matches only `16.1.0`
    /// 4. `15` does not match `16.1.0`
    /// 5. `16.0` does not match `16.1.0`
    /// 6. `16.1.0` does not match `16.1.1`
    pub fn matches(&self, version: &Version) -> bool {
        if self.major != version.major {
            return false;
        } else if self.minor.is_none() || version.minor.is_none() {
            return true;
        } else if self.minor != version.minor {
            return false;
        } else if self.release.is_none() || version.release.is_none() {
            return true;
        }

        self.release == version.release
    }
}

impl fmt::Display for Version {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let major = self.major.to_string();
        let minor = self
            .minor
            .map(|minor| format!(".{minor}"))
            .unwrap_or("".to_string());
        let release = self
            .release
            .map(|release| format!(".{release}"))
            .unwrap_or("".to_string());
        write!(formatter, "{major}{minor}{release}")
    }
}

impl FromStr for Version {
    type Err = ArchiveError;

    fn from_str(version: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = version.split('.').collect();
        let major: u64 = match parts.first() {
            Some(major) => major.parse()?,
            None => return Err(InvalidVersion(version.to_string())),
        };

        let minor: Option<u64> = match parts.get(1) {
            Some(minor) => Some(minor.parse()?),
            None => None,
        };

        let release: Option<u64> = match parts.get(2) {
            Some(release) => Some(release.parse()?),
            None => None,
        };

        if parts.len() > 3 {
            return Err(InvalidVersion(version.to_string()));
        }

        Ok(Version::new(major, minor, release))
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version = String::deserialize(deserializer)?;
        Version::from_str(&version).map_err(serde::de::Error::custom)
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    //
    // Impl tests
    //

    #[test]
    fn test_matches_all() {
        assert!(&Version::new(1, None, None).matches(&Version::new(1, None, None)));
        assert!(!&Version::new(1, None, None).matches(&Version::new(2, None, None)));

        assert!(&Version::new(1, Some(2), None).matches(&Version::new(1, Some(2), None)));
        assert!(!&Version::new(1, Some(2), None).matches(&Version::new(1, Some(3), None)));

        assert!(&Version::new(1, Some(2), Some(3)).matches(&Version::new(1, Some(2), Some(3))));
        assert!(!&Version::new(1, Some(2), Some(3)).matches(&Version::new(1, Some(2), Some(4))));

        assert!(&Version::new(1, None, None).matches(&Version::new(1, Some(2), None)));
        assert!(&Version::new(1, Some(2), None).matches(&Version::new(1, None, None)));

        assert!(&Version::new(1, Some(2), None).matches(&Version::new(1, Some(2), Some(3))));
        assert!(&Version::new(1, Some(2), Some(3)).matches(&Version::new(1, Some(2), None)));
    }

    //
    // Display tests
    //

    #[test]
    fn test_version_display() -> Result<()> {
        let version_str = "1.2.3";
        let version = Version::from_str(version_str)?;
        assert_eq!(version_str, version.to_string());
        Ok(())
    }

    #[test]
    fn test_version_display_major() -> Result<()> {
        let version_str = "1";
        let version = Version::from_str(version_str)?;
        assert_eq!(version_str, version.to_string());
        Ok(())
    }

    #[test]
    fn test_version_display_major_minor() -> Result<()> {
        let version_str = "1.2";
        let version = Version::from_str(version_str)?;
        assert_eq!(version_str, version.to_string());
        Ok(())
    }

    //
    // FromStr tests
    //

    #[test]
    fn test_version_from_str() -> Result<()> {
        let version = Version::from_str("1.2.3")?;
        assert_eq!(version.major, 1u64);
        assert_eq!(version.minor, Some(2));
        assert_eq!(version.release, Some(3));
        Ok(())
    }

    #[test]
    fn test_version_from_str_major() -> Result<()> {
        let version = Version::from_str("1")?;
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, None);
        assert_eq!(version.release, None);
        Ok(())
    }

    #[test]
    fn test_version_from_str_major_minor() -> Result<()> {
        let version = Version::from_str("1.2")?;
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, Some(2));
        assert_eq!(version.release, None);
        Ok(())
    }

    #[test]
    fn test_version_from_str_error_missing_major() {
        assert!(Version::from_str("").is_err());
    }

    #[test]
    fn test_version_from_str_error_invalid_major() {
        assert!(Version::from_str("a").is_err());
    }

    #[test]
    fn test_version_from_str_error_invalid_minor() {
        assert!(Version::from_str("1.a").is_err());
    }

    #[test]
    fn test_version_from_str_error_invalid_release() {
        assert!(Version::from_str("1.2.a").is_err());
    }

    #[test]
    fn test_version_from_str_error_too_many_parts() {
        assert!(Version::from_str("1.2.3.4").is_err());
    }

    //
    // Deserialize tests
    //

    #[test]
    fn test_version_deserialize() -> anyhow::Result<()> {
        let version = serde_json::from_str::<Version>("\"1.2.3\"")?;
        assert_eq!(version.major, 1u64);
        assert_eq!(version.minor, Some(2));
        assert_eq!(version.release, Some(3));
        Ok(())
    }

    #[test]
    fn test_version_deserialize_parse_error() {
        assert!(serde_json::from_str::<Version>("\"foo\"").is_err())
    }

    //
    // Serialize tests
    //

    #[test]
    fn test_version_serialize() -> anyhow::Result<()> {
        let version = Version::new(1, Some(2), Some(3));
        let version_str = serde_json::to_string(&version)?;
        assert_eq!(version_str, "\"1.2.3\"");
        Ok(())
    }
}
