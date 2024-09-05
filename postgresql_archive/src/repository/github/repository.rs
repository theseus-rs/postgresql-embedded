use crate::hasher::registry::HasherFn;
use crate::repository::github::models::{Asset, Release};
use crate::repository::model::Repository;
use crate::repository::Archive;
use crate::Error::{
    ArchiveHashMismatch, AssetHashNotFound, AssetNotFound, RepositoryFailure, VersionNotFound,
};
use crate::{hasher, matcher, Result};
use async_trait::async_trait;
use futures_util::StreamExt;
use http::{header, Extensions};
use human_bytes::human_bytes;
use regex::Regex;
use reqwest::{Request, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Middleware, Next};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use reqwest_tracing::TracingMiddleware;
use semver::{Version, VersionReq};
use std::env;
use std::io::Write;
use std::str::FromStr;
use std::sync::LazyLock;
use tracing::{debug, instrument, warn, Span};
use tracing_indicatif::span_ext::IndicatifSpanExt;

use url::Url;

const GITHUB_API_VERSION_HEADER: &str = "X-GitHub-Api-Version";
const GITHUB_API_VERSION: &str = "2022-11-28";

static GITHUB_TOKEN: LazyLock<Option<String>> = LazyLock::new(|| match env::var("GITHUB_TOKEN") {
    Ok(token) => {
        debug!("GITHUB_TOKEN environment variable found");
        Some(token)
    }
    Err(_) => None,
});

static USER_AGENT: LazyLock<String> = LazyLock::new(|| {
    format!(
        "{PACKAGE}/{VERSION}",
        PACKAGE = env!("CARGO_PKG_NAME"),
        VERSION = env!("CARGO_PKG_VERSION")
    )
});

/// GitHub repository.
///
/// This repository is used to interact with GitHub. The configuration url should be
/// in the format <https://github.com/owner/repository>
/// (e.g. <https://github.com/theseus-rs/postgresql-binaries>).
#[derive(Debug)]
pub struct GitHub {
    url: String,
    releases_url: String,
}

impl GitHub {
    /// Creates a new GitHub repository from the specified URL in the format
    /// <https://github.com/owner/repository>
    ///
    /// # Errors
    /// * If the URL is invalid.
    #[expect(clippy::new_ret_no_self)]
    pub fn new(url: &str) -> Result<Box<dyn Repository>> {
        let parsed_url = Url::parse(url)?;
        let path = parsed_url.path().trim_start_matches('/');
        let path_parts = path.split('/').collect::<Vec<_>>();
        let owner = (*path_parts
            .first()
            .ok_or_else(|| RepositoryFailure(format!("No owner in URL {url}")))?)
        .to_string();
        let repo = (*path_parts
            .get(1)
            .ok_or_else(|| RepositoryFailure(format!("No repo in URL {url}")))?)
        .to_string();
        let releases_url = format!("https://api.github.com/repos/{owner}/{repo}/releases");

        Ok(Box::new(Self {
            url: url.to_string(),
            releases_url,
        }))
    }

    /// Gets the version from the specified tag name.
    ///
    /// # Errors
    /// * If the version cannot be parsed.
    fn get_version_from_tag_name(tag_name: &str) -> Result<Version> {
        // Trim and prefix characters from the tag name (e.g., "v16.4.0" -> "16.4.0").
        let tag_name = tag_name.trim_start_matches(|c: char| !c.is_numeric());
        match Version::from_str(tag_name) {
            Ok(version) => Ok(version),
            Err(error) => {
                warn!("Failed to parse version {tag_name}");
                Err(error.into())
            }
        }
    }

    /// Gets the release for the specified [version requirement](VersionReq). If a release for the
    /// [version requirement](VersionReq) is not found, then an error is returned.
    ///
    /// # Errors
    /// * If the release is not found.
    #[instrument(level = "debug")]
    async fn get_release(&self, version_req: &VersionReq) -> Result<Release> {
        debug!("Attempting to locate release for version requirement {version_req}");
        let client = reqwest_client();
        let mut result: Option<Release> = None;
        let mut page = 1;

        loop {
            let request = client
                .get(&self.releases_url)
                .query(&[("page", page.to_string().as_str()), ("per_page", "100")]);
            let response = request.send().await?.error_for_status()?;
            let response_releases = response.json::<Vec<Release>>().await?;
            if response_releases.is_empty() {
                break;
            }

            for release in response_releases {
                let tag_name = release.tag_name.clone();
                let Ok(release_version) = Self::get_version_from_tag_name(tag_name.as_str()) else {
                    warn!("Failed to parse release version {tag_name}");
                    continue;
                };

                if version_req.matches(&release_version) {
                    if let Some(result_release) = &result {
                        let result_version =
                            Self::get_version_from_tag_name(result_release.tag_name.as_str())?;
                        if release_version > result_version {
                            result = Some(release);
                        }
                    } else {
                        result = Some(release);
                    }
                }
            }

            page += 1;
        }

        match result {
            Some(release) => {
                let version = Self::get_version_from_tag_name(&release.tag_name)?;
                debug!("Version {version} found for version requirement {version_req}");
                Ok(release)
            }
            None => Err(VersionNotFound(version_req.to_string())),
        }
    }

    /// Gets the asset for the specified release that passes the supplied matcher. If an asset for
    /// that passes the matcher is not found, then an [AssetNotFound] error is returned.
    ///
    /// # Errors
    /// * If the asset is not found.
    #[instrument(level = "debug", skip(version, release))]
    fn get_asset(
        &self,
        version: &Version,
        release: &Release,
    ) -> Result<(Asset, Option<Asset>, Option<HasherFn>)> {
        let matcher = matcher::registry::get(&self.url)?;
        let mut release_asset: Option<Asset> = None;
        for asset in &release.assets {
            if matcher(&self.url, asset.name.as_str(), version)? {
                release_asset = Some(asset.clone());
                break;
            }
        }

        let Some(asset) = release_asset else {
            return Err(AssetNotFound);
        };

        // Attempt to find the asset hash for the asset.
        let mut asset_hash: Option<Asset> = None;
        let mut asset_hasher_fn: Option<HasherFn> = None;
        for release_asset in &release.assets {
            let release_asset_name = release_asset.name.as_str();
            if !release_asset_name.starts_with(&asset.name) {
                continue;
            }
            let extension = release_asset_name
                .strip_prefix(format!("{}.", asset.name.as_str()).as_str())
                .unwrap_or_default();

            if let Ok(hasher_fn) = hasher::registry::get(&self.url, &extension.to_string()) {
                asset_hash = Some(release_asset.clone());
                asset_hasher_fn = Some(hasher_fn);
                break;
            }
        }

        Ok((asset, asset_hash, asset_hasher_fn))
    }
}

#[async_trait]
impl Repository for GitHub {
    #[instrument(level = "debug")]
    fn name(&self) -> &str {
        "GitHub"
    }

    #[instrument(level = "debug")]
    async fn get_version(&self, version_req: &VersionReq) -> Result<Version> {
        let release = self.get_release(version_req).await?;
        let version = Self::get_version_from_tag_name(release.tag_name.as_str())?;
        Ok(version)
    }

    #[instrument]
    #[expect(clippy::cast_precision_loss)]
    async fn get_archive(&self, version_req: &VersionReq) -> Result<Archive> {
        let release = self.get_release(version_req).await?;
        let version = Self::get_version_from_tag_name(release.tag_name.as_str())?;
        let (asset, asset_hash, asset_hasher_fn) = self.get_asset(&version, &release)?;
        let name = asset.name.clone();

        let client = reqwest_client();
        debug!("Downloading archive {}", asset.browser_download_url);
        let request = client.get(&asset.browser_download_url);
        let response = request.send().await?.error_for_status()?;
        let span = Span::current();
        let content_length = response.content_length().unwrap_or_default();
        let mut bytes = Vec::new();
        let mut source = response.bytes_stream();
        span.pb_set_length(content_length);
        while let Some(chunk) = source.next().await {
            bytes.write_all(&chunk?)?;
            span.pb_set_position(bytes.len() as u64);
        }
        debug!(
            "Archive {} downloaded: {}",
            asset.browser_download_url,
            human_bytes(bytes.len() as f64)
        );

        if let Some(asset_hash) = asset_hash {
            let archive_hash = match asset_hasher_fn {
                Some(hasher_fn) => hasher_fn(&bytes)?,
                None => return Err(AssetHashNotFound(asset.name))?,
            };
            let hash_len = archive_hash.len();

            debug!(
                "Downloading archive hash {}",
                asset_hash.browser_download_url
            );
            let request = client.get(&asset_hash.browser_download_url);
            let response = request.send().await?.error_for_status()?;
            let text = response.text().await?;
            let re = Regex::new(&format!(r"[0-9a-f]{{{hash_len}}}"))?;
            let hash = match re.find(&text) {
                Some(hash) => hash.as_str().to_string(),
                None => return Err(AssetHashNotFound(asset.name)),
            };
            debug!(
                "Archive hash {} downloaded: {}",
                asset_hash.browser_download_url,
                human_bytes(text.len() as f64)
            );

            if archive_hash != hash {
                return Err(ArchiveHashMismatch { archive_hash, hash });
            }
        }

        let archive = Archive::new(name, version, bytes);
        Ok(archive)
    }
}

/// Middleware to add headers to the request. If a GitHub token is set, then it is added as a
/// bearer token. This is used to authenticate with the GitHub API to increase the rate limit.
#[derive(Debug)]
struct GithubMiddleware;

impl GithubMiddleware {
    #[expect(clippy::unnecessary_wraps)]
    fn add_headers(request: &mut Request) -> Result<()> {
        let headers = request.headers_mut();
        headers.append(
            GITHUB_API_VERSION_HEADER,
            GITHUB_API_VERSION.parse().unwrap(),
        );
        headers.append(header::USER_AGENT, USER_AGENT.parse().unwrap());
        if let Some(token) = &*GITHUB_TOKEN {
            headers.append(
                header::AUTHORIZATION,
                format!("Bearer {token}").parse().unwrap(),
            );
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl Middleware for GithubMiddleware {
    async fn handle(
        &self,
        mut request: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        match GithubMiddleware::add_headers(&mut request) {
            Ok(()) => next.run(request, extensions).await,
            Err(error) => Err(reqwest_middleware::Error::Middleware(error.into())),
        }
    }
}

/// Creates a new reqwest client with middleware for tracing, GitHub, and retrying transient errors.
fn reqwest_client() -> ClientWithMiddleware {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    ClientBuilder::new(reqwest::Client::new())
        .with(TracingMiddleware::default())
        .with(GithubMiddleware)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::theseus::URL;

    #[test]
    fn test_name() {
        let github = GitHub::new(URL).unwrap();
        assert_eq!("GitHub", github.name());
    }

    #[test]
    fn test_get_version_from_tag_name() -> Result<()> {
        let versions = vec!["16.4.0", "v16.4.0"];
        for version in versions {
            let version = GitHub::get_version_from_tag_name(version)?;
            assert_eq!(Version::new(16, 4, 0), version);
        }

        Ok(())
    }

    #[test]
    fn test_get_version_from_tag_name_error() {
        let error = GitHub::get_version_from_tag_name("foo").unwrap_err();
        assert_eq!(
            "empty string, expected a semver version".to_string(),
            error.to_string()
        );
    }

    //
    // get_version tests
    //

    #[tokio::test]
    async fn test_get_version() -> Result<()> {
        let github = GitHub::new(URL)?;
        let version_req = VersionReq::STAR;
        let version = github.get_version(&version_req).await?;
        assert!(version > Version::new(0, 0, 0));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_specific_version() -> Result<()> {
        let github = GitHub::new(URL)?;
        let version_req = VersionReq::parse("=16.4.0")?;
        let version = github.get_version(&version_req).await?;
        assert_eq!(Version::new(16, 4, 0), version);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_specific_not_found() -> Result<()> {
        let github = GitHub::new(URL)?;
        let version_req = VersionReq::parse("=0.0.0")?;
        let error = github.get_version(&version_req).await.unwrap_err();
        assert_eq!("version not found for '=0.0.0'", error.to_string());
        Ok(())
    }

    //
    // get_archive tests
    //

    #[tokio::test]
    async fn test_get_archive() -> Result<()> {
        let github = GitHub::new(URL)?;
        let version_req = VersionReq::parse("=16.4.0")?;
        let archive = github.get_archive(&version_req).await?;
        assert_eq!(
            format!("postgresql-16.4.0-{}.tar.gz", target_triple::TARGET),
            archive.name()
        );
        assert_eq!(&Version::new(16, 4, 0), archive.version());
        assert!(!archive.bytes().is_empty());
        Ok(())
    }

    //
    // Plugin Support
    //

    /// Test that a version with a 'v' prefix is correctly parsed; this is a common convention
    /// for GitHub releases.  Use a known PostgreSQL plugin repository for the test.
    #[tokio::test]
    async fn test_get_version_with_v_prefix() -> Result<()> {
        let github = GitHub::new("https://github.com/turbot/steampipe-plugin-csv")?;
        let version_req = VersionReq::parse("=0.12.0")?;
        let version = github.get_version(&version_req).await?;
        assert_eq!(Version::new(0, 12, 0), version);
        Ok(())
    }
}
