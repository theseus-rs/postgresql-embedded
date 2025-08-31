#[cfg(not(any(
    all(target_os = "linux", target_arch = "aarch64"),
    all(target_os = "macos", target_arch = "x86_64")
)))]
#[cfg(feature = "portal-corp")]
#[tokio::test]
async fn test_extensions_portal_corp_lifecycle() -> anyhow::Result<()> {
    let installation_dir = tempfile::tempdir()?.path().to_path_buf();
    let postgresql_version = semver::VersionReq::parse("=16.4.0")?;
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

    let namespace = "portal-corp";
    let name = "pgvector_compiled";
    let version = semver::VersionReq::parse("=0.16.12")?;

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
