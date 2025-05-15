#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![deny(clippy::pedantic)]

use postgresql_archive::configuration::theseus;
use postgresql_archive::{Result, VersionReq, extract, get_archive};

#[tokio::main]
async fn main() -> Result<()> {
    let url = theseus::URL;
    let version_req = VersionReq::STAR;
    let (archive_version, archive) = get_archive(url, &version_req).await?;
    let out_dir = tempfile::tempdir()?.keep();
    extract(url, &archive, &out_dir).await?;
    println!(
        "PostgreSQL {} extracted to {}",
        archive_version,
        out_dir.to_string_lossy()
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}
