use crate::repository::maven::models::Metadata;
use crate::repository::model::Repository;
use crate::repository::Archive;
use crate::Error::{ArchiveHashMismatch, ParseError, RepositoryFailure, VersionNotFound};
use crate::{hasher, Result};
use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::header::HeaderMap;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use reqwest_tracing::TracingMiddleware;
use semver::{Version, VersionReq};
use std::env;
use std::io::Write;
use std::sync::LazyLock;
use tracing::{debug, instrument, warn};
#[cfg(feature = "indicatif")]
use tracing_indicatif::span_ext::IndicatifSpanExt;

static USER_AGENT: LazyLock<String> = LazyLock::new(|| {
    format!(
        "{PACKAGE}/{VERSION}",
        PACKAGE = env!("CARGO_PKG_NAME"),
        VERSION = env!("CARGO_PKG_VERSION")
    )
});

/// Maven repository.
///
/// This repository is used to interact with Maven repositories
/// (e.g. <https://repo1.maven.org/maven2>).
#[derive(Debug)]
pub struct Maven {
    url: String,
}

impl Maven {
    /// Creates a new Maven repository from the specified URL in the format
    /// <https://repo1.maven.org/maven2/io/zonky/test/postgres/embedded-postgres-binaries-linux-amd64>
    ///
    /// # Errors
    /// * If the URL is invalid.
    #[expect(clippy::new_ret_no_self)]
    pub fn new(url: &str) -> Result<Box<dyn Repository>> {
        Ok(Box::new(Self {
            url: url.to_string(),
        }))
    }

    /// Gets the artifact id and version that matches the specified version requirement.
    ///
    /// # Errors
    /// * If the version requirement does not match any versions.
    #[instrument(level = "debug")]
    async fn get_artifact(&self, version_req: &VersionReq) -> Result<(String, Version)> {
        debug!("Attempting to locate release for version requirement {version_req}");
        let client = reqwest_client();
        let url = format!("{}/maven-metadata.xml", self.url);
        let request = client.get(&url).headers(Self::headers());
        let response = request.send().await?.error_for_status()?;
        let text = response.text().await?;
        let metadata: Metadata =
            quick_xml::de::from_str(&text).map_err(|error| ParseError(error.to_string()))?;
        let artifact = metadata.artifact_id;
        let mut result = None;
        for version in &metadata.versioning.versions.version {
            let version = Version::parse(version)?;
            if version_req.matches(&version) {
                if let Some(result_version) = result.clone() {
                    if version > result_version {
                        result = Some(version);
                    }
                } else {
                    result = Some(version);
                }
            }
        }

        match &result {
            Some(version) => {
                debug!("Version {version} found for version requirement {version_req}");
                Ok((artifact, version.clone()))
            }
            None => Err(VersionNotFound(version_req.to_string())),
        }
    }

    /// Returns the headers for the Maven request.
    fn headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.append("User-Agent", USER_AGENT.parse().unwrap());
        headers
    }
}

#[async_trait]
impl Repository for Maven {
    #[instrument(level = "debug")]
    fn name(&self) -> &str {
        "Maven"
    }

    #[instrument(level = "debug")]
    async fn get_version(&self, version_req: &VersionReq) -> Result<Version> {
        debug!("Attempting to locate release for version requirement {version_req}");
        let (_, version) = self.get_artifact(version_req).await?;
        Ok(version)
    }

    #[instrument]
    async fn get_archive(&self, version_req: &VersionReq) -> Result<Archive> {
        let (artifact, version) = self.get_artifact(version_req).await?;
        let archive_name = format!("{artifact}-{version}.jar");
        let archive_url = format!("{url}/{version}/{artifact}-{version}.jar", url = self.url,);

        let mut hasher_result = None;
        // Try to find a hasher for the archive; the extensions are ordered by preference.
        for extension in &["sha512", "sha256", "sha1", "md5"] {
            if let Ok(hasher_fn) = hasher::registry::get(&self.url, &(*extension).to_string()) {
                hasher_result = Some((extension, hasher_fn));
            }
        }

        let Some((extension, hasher_fn)) = hasher_result else {
            return Err(RepositoryFailure(format!(
                "no hashers found for {}",
                &self.url
            )));
        };
        let archive_hash_url = format!("{archive_url}.{extension}");
        let client = reqwest_client();
        debug!("Downloading archive hash {archive_hash_url}");
        let request = client.get(&archive_hash_url).headers(Self::headers());
        let response = request.send().await?.error_for_status()?;
        let hash = response.text().await?;
        debug!("Archive hash {archive_hash_url} downloaded: {}", hash.len(),);

        debug!("Downloading archive {archive_url}");
        let request = client.get(&archive_url).headers(Self::headers());
        let response = request.send().await?.error_for_status()?;
        #[cfg(feature = "indicatif")]
        let span = tracing::Span::current();
        #[cfg(feature = "indicatif")]
        {
            let content_length = response.content_length().unwrap_or_default();
            span.pb_set_length(content_length);
        }
        let mut bytes = Vec::new();
        let mut source = response.bytes_stream();
        while let Some(chunk) = source.next().await {
            bytes.write_all(&chunk?)?;
            #[cfg(feature = "indicatif")]
            span.pb_set_position(bytes.len() as u64);
        }
        debug!("Archive {archive_url} downloaded: {}", bytes.len(),);

        let archive_hash = hasher_fn(&bytes)?;
        if archive_hash != hash {
            return Err(ArchiveHashMismatch { archive_hash, hash });
        }

        let archive = Archive::new(archive_name, version, bytes);
        Ok(archive)
    }
}

/// Creates a new reqwest client with middleware for tracing, and retrying transient errors.
fn reqwest_client() -> ClientWithMiddleware {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    ClientBuilder::new(reqwest::Client::new())
        .with(TracingMiddleware::default())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    const URL: &str = "https://repo1.maven.org/maven2/io/zonky/test/postgres/embedded-postgres-binaries-linux-amd64";

    #[test]
    fn test_name() {
        let maven = Maven::new(URL).unwrap();
        assert_eq!("Maven", maven.name());
    }

    //
    // get_version tests
    //

    #[tokio::test]
    async fn test_get_version() -> Result<()> {
        let maven = Maven::new(URL)?;
        let version_req = VersionReq::STAR;
        let version = maven.get_version(&version_req).await?;
        assert!(version > Version::new(0, 0, 0));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_specific_version() -> Result<()> {
        let maven = Maven::new(URL)?;
        let version_req = VersionReq::parse("=16.2.0")?;
        let version = maven.get_version(&version_req).await?;
        assert_eq!(Version::new(16, 2, 0), version);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_specific_not_found() -> Result<()> {
        let maven = Maven::new(URL)?;
        let version_req = VersionReq::parse("=0.0.0")?;
        let error = maven.get_version(&version_req).await.unwrap_err();
        assert_eq!("version not found for '=0.0.0'", error.to_string());
        Ok(())
    }

    //
    // get_archive tests
    //

    #[tokio::test]
    async fn test_get_archive() -> Result<()> {
        let maven = Maven::new(URL)?;
        let version = Version::new(16, 2, 0);
        let version_req = VersionReq::parse(format!("={version}").as_str())?;
        let archive = maven.get_archive(&version_req).await?;
        assert_eq!(
            format!("embedded-postgres-binaries-linux-amd64-{version}.jar"),
            archive.name()
        );
        assert_eq!(&version, archive.version());
        assert!(!archive.bytes().is_empty());
        Ok(())
    }
}
