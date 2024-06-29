#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use postgresql_archive::blocking::{extract, get_archive};
use postgresql_archive::{Result, VersionReq, THESEUS_POSTGRESQL_BINARIES_URL};

fn main() -> Result<()> {
    let url = THESEUS_POSTGRESQL_BINARIES_URL;
    let version_req = VersionReq::STAR;
    let (archive_version, archive) = get_archive(url, &version_req)?;
    let out_dir = tempfile::tempdir()?.into_path();
    extract(url, &archive, &out_dir)?;
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
