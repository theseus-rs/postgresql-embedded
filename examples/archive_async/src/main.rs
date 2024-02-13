use postgresql_archive::{extract, get_archive, Result, LATEST};

#[tokio::main]
async fn main() -> Result<()> {
    let (archive_version, archive, _hash) = get_archive(&LATEST).await?;
    let out_dir = tempfile::tempdir()?.into_path();
    extract(&archive, &out_dir).await?;
    println!(
        "PostgreSQL {} extracted to {}",
        archive_version,
        out_dir.to_string_lossy()
    );
    Ok(())
}
