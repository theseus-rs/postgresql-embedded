use anyhow::bail;
use postgresql_archive::LATEST;
use postgresql_embedded::{PostgreSQL, Result, Settings, Status};
use std::fs::{remove_dir_all, remove_file};
use test_log::test;

async fn lifecycle() -> Result<()> {
    let mut postgresql = PostgreSQL::default();
    let settings = postgresql.settings();

    // Verify that an ephemeral instance is created by default
    assert_eq!(0, settings.port);
    assert!(settings.temporary);

    let initial_statuses = [Status::NotInstalled, Status::Installed, Status::Stopped];
    assert!(initial_statuses.contains(&postgresql.status()));

    postgresql.setup().await?;
    assert_eq!(Status::Stopped, postgresql.status());

    postgresql.start().await?;
    assert_eq!(Status::Started, postgresql.status());

    let database_name = "test";
    assert!(!postgresql.database_exists(database_name).await?);
    postgresql.create_database(database_name).await?;
    assert!(postgresql.database_exists(database_name).await?);
    postgresql.drop_database(database_name).await?;

    postgresql.stop().await?;
    assert_eq!(Status::Stopped, postgresql.status());

    Ok(())
}

#[test(tokio::test)]
async fn test_lifecycle() -> Result<()> {
    lifecycle().await
}

#[test(tokio::test)]
async fn test_temporary_database() -> Result<()> {
    let settings = Settings::default();
    let data_dir = settings.data_dir.clone();
    let password_file = settings.password_file.clone();

    assert!(settings.temporary);

    {
        let mut postgresql = PostgreSQL::new(LATEST, settings);
        postgresql.setup().await?;
        postgresql.start().await?;
        assert!(data_dir.exists());
        assert!(password_file.exists());
    }

    // Verify that the data directory and password file are removed automatically when PostgreSQL is dropped
    assert!(!data_dir.exists());
    assert!(!password_file.exists());
    Ok(())
}

#[test(tokio::test)]
async fn test_persistent_database() -> Result<()> {
    let mut settings = Settings::default();
    let data_dir = settings.data_dir.clone();
    let password_file = settings.password_file.clone();

    settings.temporary = false;

    {
        let mut postgresql = PostgreSQL::new(LATEST, settings);
        postgresql.setup().await?;
        postgresql.start().await?;
        assert!(data_dir.exists());
        assert!(password_file.exists());
    }

    // Verify that the data directory and password file are retained when PostgreSQL is dropped
    assert!(data_dir.exists());
    assert!(password_file.exists());

    let _ = remove_dir_all(&data_dir);
    let _ = remove_file(&password_file);

    Ok(())
}

#[test(tokio::test)]
async fn test_persistent_database_reuse() -> Result<()> {
    let version = LATEST;
    let database_name = "test";
    let mut settings = Settings::default();
    let data_dir = settings.data_dir.clone();
    let password = settings.password.clone();
    let password_file = settings.password_file.clone();

    settings.temporary = false;

    {
        let mut postgresql = PostgreSQL::new(version, settings);
        postgresql.setup().await?;
        postgresql.start().await?;
        postgresql.create_database(database_name).await?;
        assert!(postgresql.database_exists(database_name).await?);
        postgresql.stop().await?;
    }

    // Verify that the data directory and password file are retained when PostgreSQL is dropped
    assert!(data_dir.exists());
    assert!(password_file.exists());

    let mut settings = Settings::default();
    settings.data_dir = data_dir.clone();
    settings.password = password.clone();
    settings.password_file = password_file.clone();
    settings.temporary = false;

    {
        let mut postgresql = PostgreSQL::new(version, settings);
        postgresql.setup().await?;
        postgresql.start().await?;
        assert!(postgresql.database_exists(database_name).await?);
        postgresql.stop().await?;
    }

    let _ = remove_dir_all(&data_dir);
    let _ = remove_file(&password_file);

    Ok(())
}

#[test(tokio::test)]
async fn postgres_concurrency() -> anyhow::Result<()> {
    let handle1 = tokio::spawn(lifecycle());
    let handle2 = tokio::spawn(lifecycle());
    let handle3 = tokio::spawn(lifecycle());
    match tokio::try_join!(handle1, handle2, handle3) {
        Ok(_) => {}
        Err(err) => {
            bail!("processing failed; error = {}", err);
        }
    }
    Ok(())
}

#[test(tokio::test)]
async fn test_username_setting() -> Result<()> {
    let username = "admin".to_string();
    let settings = Settings {
        username,
        ..Default::default()
    };
    let mut postgresql = PostgreSQL::new(LATEST, settings);
    postgresql.setup().await?;
    postgresql.start().await?;

    let database_name = "test";
    postgresql.create_database(database_name).await?;
    let database_exists = postgresql.database_exists(database_name).await?;
    assert!(database_exists);
    Ok(())
}
