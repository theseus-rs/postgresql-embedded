#![cfg(feature = "zip")]

use postgresql_archive::extractor::{ExtractDirectories, zip_extract};
use regex_lite::Regex;
use std::fs;
use std::io::{Cursor, Write};
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

#[test]
fn test_zip_extraction_maps_filenames_and_skips_unmatched_entries() -> anyhow::Result<()> {
    let root = tempfile::tempdir()?;
    let control_dir = root.path().join("control");
    let sql_dir = root.path().join("sql");
    let directories = ExtractDirectories::new(vec![
        (Regex::new(r"\.control$")?, control_dir.clone()),
        (Regex::new(r"\.sql$")?, sql_dir.clone()),
    ]);
    let mut archive = ZipWriter::new(Cursor::new(Vec::new()));
    for (path, contents) in [
        ("extension/éxample.control", "default_version = '1.0'"),
        ("extension/sql/éxample--1.0.sql", "SELECT 1;"),
        ("README.txt", "not an extension file"),
    ] {
        archive.start_file(path, SimpleFileOptions::default())?;
        archive.write_all(contents.as_bytes())?;
    }

    let files = zip_extract(&archive.finish()?.into_inner(), &directories)?;
    assert_eq!(
        files,
        vec![
            control_dir.join("éxample.control"),
            sql_dir.join("éxample--1.0.sql"),
        ]
    );
    assert_eq!(
        fs::read_to_string(control_dir.join("éxample.control"))?,
        "default_version = '1.0'"
    );
    assert_eq!(
        fs::read_to_string(sql_dir.join("éxample--1.0.sql"))?,
        "SELECT 1;"
    );
    assert!(!root.path().join("README.txt").exists());
    assert!(!control_dir.join("extension").exists());
    Ok(())
}
