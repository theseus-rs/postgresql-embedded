use crate::Result;
use semver::{Version, VersionReq};

/// A trait for getting the exact version from a [version requirement](VersionReq).
pub trait ExactVersion {
    /// Gets the exact version from a [version requirement](VersionReq) or `None`.
    fn exact_version(&self) -> Option<Version>;
}

impl ExactVersion for VersionReq {
    /// Gets the exact version from a [version requirement](VersionReq) or `None`.
    fn exact_version(&self) -> Option<Version> {
        if self.comparators.len() != 1 {
            return None;
        }
        let comparator = self.comparators.first()?;
        if comparator.op != semver::Op::Exact {
            return None;
        }
        let minor = comparator.minor?;
        let patch = comparator.patch?;
        let version = Version::new(comparator.major, minor, patch);
        Some(version)
    }
}

/// A trait for getting the exact version requirement from a [version](Version).
pub trait ExactVersionReq {
    /// Gets the exact version requirement from a [version](Version).
    ///
    /// # Errors
    /// * If the version requirement cannot be parsed.
    fn exact_version_req(&self) -> Result<VersionReq>;
}

impl ExactVersionReq for Version {
    /// Gets the exact version requirement from a [version](Version).
    ///
    /// # Errors
    /// * If the version requirement cannot be parsed.
    fn exact_version_req(&self) -> Result<VersionReq> {
        let version = format!("={self}");
        let version_req = VersionReq::parse(&version)?;
        Ok(version_req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn test_exact_version_star() {
        let version_req = VersionReq::STAR;
        assert_eq!(None, version_req.exact_version());
    }

    #[test]
    fn test_exact_version_greater_than() -> Result<()> {
        let version_req = VersionReq::parse(">16")?;
        assert_eq!(None, version_req.exact_version());
        Ok(())
    }

    #[test]
    fn test_exact_version_full_no_equals() -> Result<()> {
        let version_req = VersionReq::parse("16.4.0")?;
        assert_eq!(None, version_req.exact_version());
        Ok(())
    }

    #[test]
    fn test_exact_version_full_equals() -> Result<()> {
        let version_req = VersionReq::parse("=16.4.0")?;
        let version = Version::new(16, 4, 0);
        assert_eq!(Some(version), version_req.exact_version());
        Ok(())
    }

    #[test]
    fn test_exact_version_major_minor() -> Result<()> {
        let version_req = VersionReq::parse("=16.4")?;
        assert_eq!(None, version_req.exact_version());
        Ok(())
    }

    #[test]
    fn test_exact_version_major() -> Result<()> {
        let version_req = VersionReq::parse("=16")?;
        assert_eq!(None, version_req.exact_version());
        Ok(())
    }

    #[test]
    fn test_exact_version_range() -> Result<()> {
        let version_req = VersionReq::parse(">= 16, < 17")?;
        assert_eq!(None, version_req.exact_version());
        Ok(())
    }

    #[test]
    fn test_exact_version_req_not_equal() -> Result<()> {
        let version = Version::new(1, 2, 3);
        assert_ne!(VersionReq::parse("=1.0.0")?, version.exact_version_req()?);
        Ok(())
    }

    #[test]
    fn test_exact_version_req_major_minor_patch() -> Result<()> {
        let version = Version::new(16, 4, 0);
        assert_eq!(VersionReq::parse("=16.4.0")?, version.exact_version_req()?);
        Ok(())
    }

    #[test]
    fn test_exact_version_prerelease() -> Result<()> {
        let version = Version::parse("1.2.3-alpha")?;
        assert_eq!(
            VersionReq::parse("=1.2.3-alpha")?,
            version.exact_version_req()?
        );
        Ok(())
    }
}
