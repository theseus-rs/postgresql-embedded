#![allow(dead_code)]

use anyhow::Result;
use postgresql_archive::repository::github::repository::GitHub;
use postgresql_archive::{VersionReq, matcher};
use postgresql_archive::{get_archive, repository};
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, LazyLock, Mutex};
use std::{env, fs};
use url::Url;

static CUSTOM_REPO_URL: LazyLock<Arc<Mutex<String>>> =
    LazyLock::new(|| Arc::new(Mutex::new(String::new())));

/// Stage the PostgreSQL archive when the `bundled` feature is enabled so that
/// it can be included in the final binary. This is useful for creating a
/// self-contained binary that does not require the PostgreSQL archive to be
/// downloaded at runtime.
pub(crate) async fn stage_postgresql_archive() -> Result<()> {
    #[cfg(feature = "theseus")]
    let default_releases_url = postgresql_archive::configuration::theseus::URL.to_string();
    #[cfg(not(feature = "theseus"))]
    let default_releases_url = String::new();
    let releases_url = env::var("POSTGRESQL_RELEASES_URL").unwrap_or(default_releases_url);
    println!("PostgreSQL releases URL: {releases_url}");
    let postgres_version_req = env::var("POSTGRESQL_VERSION").unwrap_or("*".to_string());
    let version_req = VersionReq::from_str(postgres_version_req.as_str())?;
    println!("PostgreSQL version: {postgres_version_req}");
    println!("Target: {}", target_triple::TARGET);
    {
        let mut custom_repo_url = CUSTOM_REPO_URL.lock().unwrap();
        custom_repo_url.push_str(&releases_url);
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    println!("OUT_DIR: {:?}", out_dir);

    let mut archive_version_file = out_dir.clone();
    archive_version_file.push("postgresql.version");
    let mut archive_file = out_dir.clone();
    archive_file.push("postgresql.tar.gz");

    if archive_version_file.exists() && archive_file.exists() {
        println!("PostgreSQL archive exists: {:?}", archive_file);
        return Ok(());
    }

    // let str = releases_url.clone();
    // let s = str.leak();
    register_github_repository()?;
    let (asset_version, archive) = get_archive(&releases_url, &version_req).await?;

    fs::write(archive_version_file.clone(), asset_version.to_string())?;
    let mut file = File::create(archive_file.clone())?;
    file.write_all(&archive)?;
    file.sync_data()?;
    println!("PostgreSQL archive written to: {:?}", archive_file);

    Ok(())
}

fn register_github_repository() -> Result<()> {
    repository::registry::register(
        |url| {
            let parsed_url = Url::parse(url)?;
            let host = parsed_url.host_str().unwrap_or_default();
            Ok(host.ends_with("github.com"))
        },
        Box::new(GitHub::new),
    )?;

    matcher::registry::register(
        |url| {
            CUSTOM_REPO_URL
                .lock()
                .map_err(|err| postgresql_archive::Error::PoisonedLock(err.to_string()))
                .map(|custom_repo_url| url == custom_repo_url.deref())
        },
        postgresql_archive::configuration::matcher,
    )?;

    Ok(())
}
