use postgresql_archive::configuration::zonky;
use postgresql_embedded::{PostgreSQL, Result, Settings, Status};

#[tokio::test]
async fn test_zonky() -> Result<()> {
    let settings = Settings {
        releases_url: zonky::URL.to_string(),
        ..Default::default()
    };
    let mut postgresql = PostgreSQL::new(settings);
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
