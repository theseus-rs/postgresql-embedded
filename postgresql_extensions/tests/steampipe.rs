#[cfg(any(target_os = "linux", target_os = "macos"))]
#[cfg(feature = "steampipe")]
#[tokio::test]
async fn test_extensions_steampipe_lifecycle() -> anyhow::Result<()> {
    let installation_dir = tempfile::tempdir()?.path().to_path_buf();
    let postgresql_version = semver::VersionReq::parse("=15.7.0")?;
    let settings = postgresql_embedded::Settings {
        version: postgresql_version.clone(),
        installation_dir: installation_dir.clone(),
        ..Default::default()
    };
    let mut postgresql = postgresql_embedded::PostgreSQL::new(settings);

    postgresql.setup().await?;

    let settings = postgresql.settings();
    // Skip the test if the PostgreSQL version does not match; when testing with the 'bundled'
    // feature, the version may vary and the test will fail.
    if settings.version != postgresql_version {
        return Ok(());
    }

    let namespace = "steampipe";
    let name = "csv";
    let version = semver::VersionReq::parse("=0.12.0")?;

    let installed_extensions = postgresql_extensions::get_installed_extensions(settings).await?;
    assert!(installed_extensions.is_empty());

    postgresql_extensions::install(settings, namespace, name, &version).await?;

    let installed_extensions = postgresql_extensions::get_installed_extensions(settings).await?;
    assert!(!installed_extensions.is_empty());

    postgresql_extensions::uninstall(settings, namespace, name).await?;

    let installed_extensions = postgresql_extensions::get_installed_extensions(settings).await?;
    assert!(installed_extensions.is_empty());

    tokio::fs::remove_dir_all(&installation_dir).await?;
    Ok(())
}
