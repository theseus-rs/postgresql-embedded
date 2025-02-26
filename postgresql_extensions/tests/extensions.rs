use anyhow::Result;
use postgresql_extensions::get_available_extensions;

#[tokio::test]
async fn test_get_available_extensions() -> Result<()> {
    let extensions = get_available_extensions().await?;
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

#[cfg(all(target_os = "linux", feature = "tensor-chord"))]
#[tokio::test]
async fn test_lifecycle() -> Result<()> {
    let installation_dir = tempfile::tempdir()?.path().to_path_buf();
    let settings = postgresql_embedded::Settings {
        version: postgresql_embedded::VersionReq::parse("=16.4.0")?,
        installation_dir: installation_dir.clone(),
        ..Default::default()
    };
    let mut postgresql = postgresql_embedded::PostgreSQL::new(settings);

    postgresql.setup().await?;

    let settings = postgresql.settings();
    let namespace = "tensor-chord";
    let name = "pgvecto.rs";
    let version = semver::VersionReq::parse("=0.3.0")?;

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
