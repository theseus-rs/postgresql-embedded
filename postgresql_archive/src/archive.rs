//! Manage PostgreSQL archive
#![allow(dead_code)]

use crate::error::Error::{AssetHashNotFound, AssetNotFound, ReleaseNotFound, Unexpected};
use crate::error::Result;
use crate::github::{Asset, Release};
use crate::version::Version;
use crate::Error::ArchiveHashMismatch;
use bytes::Bytes;
use flate2::bufread::GzDecoder;
use human_bytes::human_bytes;
use num_format::{Locale, ToFormattedString};
use regex::Regex;
use reqwest::{header, Request, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Middleware, Next};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use reqwest_tracing::TracingMiddleware;
use sha2::{Digest, Sha256};
use std::fs::{create_dir_all, remove_dir_all, remove_file, rename, File};
use std::io::{copy, BufReader, Cursor};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use tar::Archive;
use task_local_extensions::Extensions;
use tracing::{debug, instrument, warn};

const GITHUB_API_VERSION_HEADER: &str = "X-GitHub-Api-Version";
const GITHUB_API_VERSION: &str = "2022-11-28";

lazy_static! {
    static ref GITHUB_TOKEN: Option<String> = match std::env::var("GITHUB_TOKEN") {
        Ok(token) => {
            debug!("GITHUB_TOKEN environment variable found");
            Some(token)
        }
        Err(_) => None,
    };
}

lazy_static! {
    static ref USER_AGENT: String = format!(
        "{PACKAGE}/{VERSION}",
        PACKAGE = env!("CARGO_PKG_NAME"),
        VERSION = env!("CARGO_PKG_VERSION")
    );
}

/// Middleware to add GitHub headers to the request. If a GitHub token is set, then it is added as a
/// bearer token. This is used to authenticate with the GitHub API to increase the rate limit.
#[derive(Debug)]
struct GithubMiddleware;

impl GithubMiddleware {
    fn add_github_headers(&self, request: &mut Request) -> Result<()> {
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
        match self.add_github_headers(&mut request) {
            Ok(_) => next.run(request, extensions).await,
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

/// Gets a release from GitHub for a given [version](Version) of PostgreSQL. If a release for the
/// [version](Version) is not found, then a [ReleaseNotFound] error is returned.
#[instrument(level = "debug")]
async fn get_release(version: &Version) -> Result<Release> {
    let url = "https://api.github.com/repos/theseus-rs/postgresql-binaries/releases";
    let client = reqwest_client();

    debug!("Attempting to locate release for version {version}");

    if version.minor.is_some() && version.release.is_some() {
        let request = client.get(format!("{url}/tags/{version}"));
        let response = request.send().await?.error_for_status()?;
        let release = response.json::<Release>().await?;

        debug!("Release found for version {version}");
        return Ok(release);
    }

    let mut result: Option<Release> = None;
    let mut page = 1;

    loop {
        let request = client
            .get(url)
            .query(&[("page", page.to_string().as_str()), ("per_page", "100")]);
        let response = request.send().await?.error_for_status()?;
        let response_releases = response.json::<Vec<Release>>().await?;
        if response_releases.is_empty() {
            break;
        }

        for release in response_releases {
            let release_version = match Version::from_str(&release.tag_name) {
                Ok(release_version) => release_version,
                Err(_) => {
                    warn!("Failed to parse release version {}", release.tag_name);
                    continue;
                }
            };

            if version.matches(&release_version) {
                match &result {
                    Some(result_release) => {
                        let result_version = Version::from_str(&result_release.tag_name)?;
                        if release_version > result_version {
                            result = Some(release);
                        }
                    }
                    None => {
                        result = Some(release);
                    }
                }
            }
        }

        page += 1;
    }

    match result {
        Some(release) => {
            let release_version = Version::from_str(&release.tag_name)?;
            debug!("Release {release_version} found for version {version}");
            Ok(release)
        }
        None => Err(ReleaseNotFound(version.to_string())),
    }
}

/// Gets the version of PostgreSQL for the specified [version](Version).  If the version minor or release is not
/// specified, then the latest version is returned. If a release for the [version](Version) is not found, then a
/// [ReleaseNotFound] error is returned.
#[instrument(level = "debug")]
pub async fn get_version(version: &Version) -> Result<Version> {
    let release = get_release(version).await?;
    Version::from_str(&release.tag_name)
}

/// Gets the assets for a given [version](Version) of PostgreSQL and
/// [target](https://doc.rust-lang.org/nightly/rustc/platform-support.html).
/// If the [version](Version) or [target](https://doc.rust-lang.org/nightly/rustc/platform-support.html)
/// is not found, then an [error](crate::error::Error) is returned.
///
/// Two assets are returned. The first [asset](Asset) is the archive, and the second [asset](Asset) is the archive hash.
#[instrument(level = "debug", skip(target))]
async fn get_asset<S: AsRef<str>>(version: &Version, target: S) -> Result<(Version, Asset, Asset)> {
    let release = get_release(version).await?;
    let asset_version = Version::from_str(&release.tag_name)?;
    let mut asset: Option<Asset> = None;
    let mut asset_hash: Option<Asset> = None;
    let asset_name = format!("postgresql-{}-{}.tar.gz", asset_version, target.as_ref());
    let asset_hash_name = format!("{asset_name}.sha256");

    for release_asset in release.assets {
        if release_asset.name == asset_name {
            asset = Some(release_asset);
        } else if release_asset.name == asset_hash_name {
            asset_hash = Some(release_asset);
        }

        if asset.is_some() && asset_hash.is_some() {
            break;
        }
    }

    match (asset, asset_hash) {
        (Some(asset), Some(asset_hash)) => Ok((asset_version, asset, asset_hash)),
        (None, _) => Err(AssetNotFound(asset_name.to_string())),
        (_, None) => Err(AssetNotFound(asset_name.to_string())),
    }
}

/// Gets the archive for a given [version](Version) of PostgreSQL for the current target.
/// If the [version](Version) is not found for this target, then an
/// [error](crate::error::Error) is returned.
///
/// Returns the archive version and bytes.
#[instrument]
pub async fn get_archive(version: &Version) -> Result<(Version, Bytes)> {
    get_archive_for_target(version, target_triple::TARGET).await
}

/// Gets the archive for a given [version](Version) of PostgreSQL and
/// [target](https://doc.rust-lang.org/nightly/rustc/platform-support.html).
/// If the [version](Version) or [target](https://doc.rust-lang.org/nightly/rustc/platform-support.html)
/// is not found, then an [error](crate::error::Error) is returned.
///
/// Returns the archive version and bytes.
#[instrument(level = "debug", skip(target))]
pub async fn get_archive_for_target<S: AsRef<str>>(
    version: &Version,
    target: S,
) -> Result<(Version, Bytes)> {
    let (asset_version, asset, asset_hash) = get_asset(version, target).await?;

    debug!(
        "Downloading archive hash {}",
        asset_hash.browser_download_url
    );
    let client = reqwest_client();
    let request = client.get(&asset_hash.browser_download_url);
    let response = request.send().await?.error_for_status()?;
    let text = response.text().await?;
    let re = Regex::new(r"[0-9a-f]{64}")?;
    let hash = match re.find(&text) {
        Some(hash) => hash.as_str().to_string(),
        None => return Err(AssetHashNotFound(asset.name)),
    };
    debug!(
        "Archive hash {} downloaded: {}",
        asset_hash.browser_download_url,
        human_bytes(text.len() as f64)
    );

    debug!("Downloading archive {}", asset.browser_download_url);
    let request = client.get(&asset.browser_download_url);
    let response = request.send().await?.error_for_status()?;
    let archive: Bytes = response.bytes().await?;
    debug!(
        "Archive {} downloaded: {}",
        asset.browser_download_url,
        human_bytes(archive.len() as f64)
    );

    let mut hasher = Sha256::new();
    hasher.update(&archive);
    let archive_hash = hex::encode(hasher.finalize());

    if archive_hash != hash {
        return Err(ArchiveHashMismatch { archive_hash, hash });
    }

    Ok((asset_version, archive))
}

/// Acquires a lock file in the [out_dir](Path) to prevent multiple processes from extracting the
/// archive at the same time.
#[instrument(level = "debug")]
fn acquire_lock(out_dir: &Path) -> Result<PathBuf> {
    let lock_file = out_dir.join("postgresql-archive.lock");

    if lock_file.is_file() {
        let metadata = lock_file.metadata()?;
        let created = metadata.created()?;

        if created.elapsed()?.as_secs() > 300 {
            warn!(
                "Stale lock file detected; removing file to attempt process recovery: {}",
                lock_file.to_string_lossy()
            );
            remove_file(&lock_file)?;
        }
    }

    debug!(
        "Attempting to acquire lock: {}",
        lock_file.to_string_lossy()
    );

    for _ in 0..30 {
        let lock = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&lock_file);

        match lock {
            Ok(_) => {
                debug!("Lock acquired: {}", lock_file.to_string_lossy());
                return Ok(lock_file);
            }
            Err(error) => {
                warn!("unable to acquire lock: {error}");
                sleep(Duration::from_secs(1));
            }
        }
    }

    Err(Unexpected("Failed to acquire lock".to_string()))
}

/// Extracts the compressed tar [bytes](Bytes) to the [out_dir](Path).
#[instrument]
pub async fn extract(bytes: &Bytes, out_dir: &Path) -> Result<()> {
    let input = BufReader::new(Cursor::new(bytes));
    let decoder = GzDecoder::new(input);
    let mut archive = Archive::new(decoder);
    let mut files = 0;
    let mut extracted_bytes = 0;

    let parent_dir = match out_dir.parent() {
        Some(parent) => parent,
        None => {
            debug!("No parent directory for {}", out_dir.to_string_lossy());
            out_dir
        }
    };
    create_dir_all(parent_dir)?;

    let lock_file = acquire_lock(parent_dir)?;
    // If the directory already exists, then the archive has already been
    // extracted by another process.
    if out_dir.exists() {
        debug!(
            "Directory already exists {}; skipping extraction: ",
            out_dir.to_string_lossy()
        );
        remove_file(&lock_file)?;
        return Ok(());
    }

    let extract_dir = tempfile::tempdir()?.into_path();
    create_dir_all(&extract_dir)?;

    debug!("Extracting archive to {}", extract_dir.to_string_lossy());

    for archive_entry in archive.entries()? {
        let mut entry = archive_entry?;
        let entry_header = entry.header();
        let entry_type = entry_header.entry_type();
        let entry_size = entry_header.size()?;
        #[cfg(unix)]
        let file_mode = entry_header.mode()?;

        let entry_header_path = entry_header.path()?.to_path_buf();
        let prefix = match entry_header_path.components().next() {
            Some(component) => component.as_os_str().to_str().unwrap_or_default(),
            None => {
                return Err(Unexpected(
                    "Failed to get file header path prefix".to_string(),
                ));
            }
        };
        let stripped_entry_header_path = entry_header_path.strip_prefix(prefix)?.to_path_buf();
        let mut entry_name = extract_dir.to_path_buf();
        entry_name.push(stripped_entry_header_path);

        if entry_type.is_dir() || entry_name.is_dir() {
            create_dir_all(&entry_name)?;
        } else if entry_type.is_file() {
            let mut output_file = File::create(&entry_name)?;
            copy(&mut entry, &mut output_file)?;

            files += 1;
            extracted_bytes += entry_size;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                output_file.set_permissions(std::fs::Permissions::from_mode(file_mode))?;
            }
        } else if entry_type.is_symlink() {
            #[cfg(unix)]
            if let Some(symlink_target) = entry.link_name()? {
                let symlink_path = entry_name;
                std::os::unix::fs::symlink(symlink_target.as_ref(), symlink_path)?;
            }
        }
    }

    if out_dir.exists() {
        debug!(
            "Directory already exists {}; skipping name and removing extraction directory: {}",
            out_dir.to_string_lossy(),
            extract_dir.to_string_lossy()
        );
        remove_dir_all(&extract_dir)?;
    } else {
        debug!(
            "Renaming {} to {}",
            extract_dir.to_string_lossy(),
            out_dir.to_string_lossy()
        );
        rename(extract_dir, out_dir)?;
    }

    if lock_file.is_file() {
        debug!("Removing lock file: {}", lock_file.to_string_lossy());
        remove_file(lock_file)?;
    }

    debug!(
        "Extracting {} files totalling {}",
        files.to_formatted_string(&Locale::en),
        human_bytes(extracted_bytes as f64)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    /// Use a known, fully defined version to speed up test execution
    const VERSION: Version = Version::new(16, Some(1), Some(0));
    const INVALID_VERSION: Version = Version::new(1, Some(0), Some(0));

    #[test(tokio::test)]
    async fn test_get_release() -> Result<()> {
        let _ = get_release(&VERSION).await?;
        Ok(())
    }

    #[test(tokio::test)]
    async fn test_get_release_version_not_found() -> Result<()> {
        let release = get_release(&INVALID_VERSION).await;
        assert!(release.is_err());
        Ok(())
    }

    #[test(tokio::test)]
    async fn test_get_asset() -> Result<()> {
        let target_triple = "x86_64-unknown-linux-musl".to_string();
        let (asset_version, asset, asset_hash) = get_asset(&VERSION, &target_triple).await?;
        assert!(asset_version.matches(&VERSION));
        assert!(asset.name.contains(&target_triple));
        assert!(asset_hash.name.contains(&target_triple));
        assert!(asset_hash.name.starts_with(asset.name.as_str()));
        assert!(asset_hash.name.ends_with(".sha256"));
        Ok(())
    }

    #[test(tokio::test)]
    async fn test_get_asset_version_not_found() -> Result<()> {
        let target_triple = "x86_64-unknown-linux-musl".to_string();
        let result = get_asset(&INVALID_VERSION, &target_triple).await;
        assert!(result.is_err());
        Ok(())
    }

    #[test(tokio::test)]
    async fn test_get_asset_target_not_found() -> Result<()> {
        let target_triple = "wasm64-unknown-unknown".to_string();
        let result = get_asset(&VERSION, &target_triple).await;
        assert!(result.is_err());
        Ok(())
    }
}
