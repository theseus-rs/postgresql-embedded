use anyhow::Result;
use postgresql_embedded::{PostgreSQL, Settings};
use postgresql_extensions::{
    get_available_extensions, get_installed_extensions, install, uninstall,
};
use semver::VersionReq;
use tokio::fs::remove_dir_all;

#[tokio::test]
async fn test_get_available_extensions() -> Result<()> {
    let extensions = get_available_extensions().await?;
    #[cfg(feature = "steampipe")]
    assert!(extensions
        .iter()
        .any(|extension| extension.namespace() == "steampipe"));
    #[cfg(feature = "tensor-chord")]
    assert!(extensions
        .iter()
        .any(|extension| extension.namespace() == "tensor-chord"));
    Ok(())
}

#[cfg(all(target_os = "linux", feature = "tensor-chord"))]
#[tokio::test]
async fn test_lifecycle() -> Result<()> {
    let installation_dir = tempfile::tempdir()?.path().to_path_buf();
    let settings = Settings {
        installation_dir: installation_dir.clone(),
        ..Default::default()
    };
    let mut postgresql = PostgreSQL::new(settings);

    postgresql.setup().await?;

    let settings = postgresql.settings();
    let namespace = "tensor-chord";
    let name = "pgvecto.rs";
    let version = VersionReq::parse("=0.3.0")?;

    let installed_extensions = get_installed_extensions(settings).await?;
    assert!(installed_extensions.is_empty());

    install(settings, namespace, name, &version).await?;

    let installed_extensions = get_installed_extensions(settings).await?;
    assert!(!installed_extensions.is_empty());

    uninstall(settings, namespace, name).await?;

    let installed_extensions = get_installed_extensions(settings).await?;
    assert!(installed_extensions.is_empty());

    remove_dir_all(&installation_dir).await?;
    Ok(())
}
