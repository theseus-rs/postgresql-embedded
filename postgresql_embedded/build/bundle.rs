#![allow(dead_code)]

use anyhow::Result;
use postgresql_archive::{get_archive, DEFAULT_RELEASES_URL};
use postgresql_archive::{Version, LATEST};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::{env, fs};

/// Stage the PostgreSQL archive when the `bundled` feature is enabled so that
/// it can be included in the final binary. This is useful for creating a
/// self-contained binary that does not require the PostgreSQL archive to be
/// downloaded at runtime.
pub(crate) async fn stage_postgresql_archive() -> Result<()> {
    let postgres_version = env::var("POSTGRESQL_VERSION").unwrap_or(LATEST.to_string());
    let version = Version::from_str(postgres_version.as_str())?;
    println!("PostgreSQL version: {postgres_version}");

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

    let (asset_version, archive) = get_archive(DEFAULT_RELEASES_URL, &version).await?;

    fs::write(archive_version_file.clone(), asset_version.to_string())?;
    let mut file = File::create(archive_file.clone())?;
    file.write_all(&archive)?;
    file.sync_data()?;
    println!("PostgreSQL archive written to: {:?}", archive_file);

    Ok(())
}
