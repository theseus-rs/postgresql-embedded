use crate::configuration::zonky::matcher::{get_arch, get_os};
use crate::repository::maven::repository::Maven;
use crate::repository::model::Repository;
use crate::repository::Archive;
use crate::Result;
use async_trait::async_trait;
use semver::{Version, VersionReq};
use tracing::instrument;

/// Zonky repository.
///
/// This repository is used to interact with Zonky Maven repositories
/// (e.g. <https://repo1.maven.org/maven2/io/zonky/test/postgres">).
#[derive(Debug)]
pub struct Zonky {
    maven: Box<dyn Repository>,
}

const MAVEN_URL: &str = "https://repo1.maven.org/maven2/io/zonky/test/postgres";

impl Zonky {
    /// Creates a new Zonky repository from the specified URL in the format
    /// <https://github.com/zonkyio/embedded-postgres-binaries>
    ///
    /// # Errors
    /// * If the URL is invalid.
    #[expect(clippy::new_ret_no_self)]
    pub fn new(_url: &str) -> Result<Box<dyn Repository>> {
        let os = get_os();
        let arch = get_arch();
        let archive = format!("embedded-postgres-binaries-{os}-{arch}");
        let url = format!("{MAVEN_URL}/{archive}");
        let maven = Maven::new(url.as_str())?;
        Ok(Box::new(Zonky { maven }))
    }
}

#[async_trait]
impl Repository for Zonky {
    #[instrument(level = "debug")]
    fn name(&self) -> &str {
        "Zonky"
    }

    #[instrument(level = "debug")]
    async fn get_version(&self, version_req: &VersionReq) -> Result<Version> {
        self.maven.get_version(version_req).await
    }

    #[instrument]
    async fn get_archive(&self, version_req: &VersionReq) -> Result<Archive> {
        self.maven.get_archive(version_req).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::zonky;

    #[test]
    fn test_name() {
        let zonky = Zonky::new(zonky::URL).unwrap();
        assert_eq!("Zonky", zonky.name());
    }

    //
    // get_version tests
    //

    #[tokio::test]
    async fn test_get_version() -> Result<()> {
        let maven = Zonky::new(zonky::URL)?;
        let version_req = VersionReq::STAR;
        let version = maven.get_version(&version_req).await?;
        assert!(version > Version::new(0, 0, 0));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_specific_version() -> Result<()> {
        let zonky = Zonky::new(zonky::URL)?;
        let version_req = VersionReq::parse("=16.2.0")?;
        let version = zonky.get_version(&version_req).await?;
        assert_eq!(Version::new(16, 2, 0), version);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_specific_not_found() -> Result<()> {
        let zonky = Zonky::new(zonky::URL)?;
        let version_req = VersionReq::parse("=0.0.0")?;
        let error = zonky.get_version(&version_req).await.unwrap_err();
        assert_eq!("version not found for '=0.0.0'", error.to_string());
        Ok(())
    }

    //
    // get_archive tests
    //

    #[tokio::test]
    async fn test_get_archive() -> Result<()> {
        let zonky = Zonky::new(zonky::URL)?;
        let os = get_os();
        let arch = get_arch();
        let version = Version::new(16, 2, 0);
        let version_req = VersionReq::parse(format!("={version}").as_str())?;
        let archive = zonky.get_archive(&version_req).await?;
        assert_eq!(
            format!("embedded-postgres-binaries-{os}-{arch}-{version}.jar"),
            archive.name()
        );
        assert_eq!(&version, archive.version());
        assert!(!archive.bytes().is_empty());
        Ok(())
    }
}
