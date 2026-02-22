use postgresql_embedded::{PostgreSQL, Result, SettingsBuilder};

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<()> {
    let socket_dir = tempfile::tempdir().expect("failed to create temp dir for socket");

    let settings = SettingsBuilder::new()
        .socket_dir(socket_dir.path().to_path_buf())
        .build();

    let mut postgresql = PostgreSQL::new(settings);
    postgresql.setup().await?;
    postgresql.start().await?;

    let port = postgresql.settings().port;
    let socket_file = socket_dir.path().join(format!(".s.PGSQL.{port}"));
    println!("PostgreSQL is listening on Unix socket: {socket_file:?}");

    let database_name = "test";
    postgresql.create_database(database_name).await?;
    println!("Created database '{database_name}'");

    let exists = postgresql.database_exists(database_name).await?;
    println!("Database '{database_name}' exists: {exists}");

    postgresql.drop_database(database_name).await?;
    println!("Dropped database '{database_name}'");

    postgresql.stop().await?;
    println!("PostgreSQL stopped");

    Ok(())
}

#[cfg(not(unix))]
fn main() {
    eprintln!("Unix socket support is only available on Unix platforms");
}

#[cfg(test)]
#[cfg(unix)]
mod test {
    use super::*;

    #[test]
    fn test_unix_socket_main() -> Result<()> {
        main()
    }
}
