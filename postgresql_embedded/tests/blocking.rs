#[cfg(feature = "blocking")]
use postgresql_embedded::blocking::PostgreSQL;
#[cfg(feature = "blocking")]
use postgresql_embedded::{Result, Status};
#[cfg(feature = "blocking")]
use test_log::test;

#[cfg(feature = "blocking")]
#[test]
fn test_lifecycle() -> Result<()> {
    let mut postgresql = PostgreSQL::default();

    let initial_statuses = [Status::NotInstalled, Status::Installed, Status::Stopped];
    assert!(initial_statuses.contains(&postgresql.status()));

    postgresql.setup()?;
    assert_eq!(Status::Stopped, postgresql.status());

    postgresql.start()?;
    assert_eq!(Status::Started, postgresql.status());

    let database_name = "test";
    postgresql.create_database(database_name)?;
    assert!(postgresql.database_exists(database_name)?);
    postgresql.drop_database(database_name)?;

    postgresql.stop()?;
    assert_eq!(Status::Stopped, postgresql.status());

    Ok(())
}
