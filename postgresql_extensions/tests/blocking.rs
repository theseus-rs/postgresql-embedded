#[cfg(feature = "blocking")]
use test_log::test;

#[cfg(feature = "blocking")]
#[test]
fn test_get_available_extensions() -> anyhow::Result<()> {
    let extensions = postgresql_extensions::blocking::get_available_extensions()?;
    #[cfg(feature = "steampipe")]
    assert!(
        extensions
            .iter()
            .any(|extension| extension.namespace() == "steampipe")
    );
    #[cfg(feature = "tensor-chord")]
    assert!(
        extensions
            .iter()
            .any(|extension| extension.namespace() == "tensor-chord")
    );
    Ok(())
}

#[cfg(all(target_os = "linux", feature = "blocking", feature = "tensor-chord"))]
#[test]
fn test_extensions_blocking_lifecycle() -> anyhow::Result<()> {
    let installation_dir = tempfile::tempdir()?.path().to_path_buf();
    let postgresql_version = semver::VersionReq::parse("=16.4.0")?;
    let settings = postgresql_embedded::Settings {
        version: postgresql_version.clone(),
        installation_dir: installation_dir.clone(),
        ..Default::default()
    };
    let mut postgresql = postgresql_embedded::blocking::PostgreSQL::new(settings);

    postgresql.setup()?;

    let settings = postgresql.settings();
    // Skip the test if the PostgreSQL version does not match; when testing with the 'bundled'
    // feature, the version may vary and the test will fail.
    if settings.version != postgresql_version {
        return Ok(());
    }

    let namespace = "tensor-chord";
    let name = "pgvecto.rs";
    let version = semver::VersionReq::parse("=0.3.0")?;

    let installed_extensions = postgresql_extensions::blocking::get_installed_extensions(settings)?;
    assert!(installed_extensions.is_empty());

    postgresql_extensions::blocking::install(settings, namespace, name, &version)?;

    let installed_extensions = postgresql_extensions::blocking::get_installed_extensions(settings)?;
    assert!(!installed_extensions.is_empty());

    postgresql_extensions::blocking::uninstall(settings, namespace, name)?;

    let installed_extensions = postgresql_extensions::blocking::get_installed_extensions(settings)?;
    assert!(installed_extensions.is_empty());

    std::fs::remove_dir_all(&installation_dir)?;
    Ok(())
}
