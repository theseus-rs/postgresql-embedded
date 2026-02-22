#[cfg(unix)]
mod unix_socket_tests {
    use postgresql_embedded::{PostgreSQL, Result, SettingsBuilder, Status};
    use sqlx::{PgPool, Row};
    use std::path::PathBuf;
    use test_log::test;

    #[test(tokio::test)]
    async fn test_unix_socket_lifecycle() -> Result<()> {
        let socket_dir = tempfile::tempdir().expect("failed to create temp dir for socket");
        let socket_path = socket_dir.path().to_path_buf();

        let settings = SettingsBuilder::new()
            .socket_dir(socket_path.clone())
            .build();

        let mut postgresql = PostgreSQL::new(settings);

        postgresql.setup().await?;
        postgresql.start().await?;

        assert_eq!(Status::Started, postgresql.status());

        // Verify the socket file exists (PostgreSQL creates .s.PGSQL.<port> in the socket dir)
        let port = postgresql.settings().port;
        let socket_file = socket_path.join(format!(".s.PGSQL.{port}"));
        assert!(
            socket_file.exists(),
            "Expected socket file at {socket_file:?}"
        );

        let database_name = "test";
        assert!(!postgresql.database_exists(database_name).await?);
        postgresql.create_database(database_name).await?;
        assert!(postgresql.database_exists(database_name).await?);
        postgresql.drop_database(database_name).await?;
        assert!(!postgresql.database_exists(database_name).await?);

        postgresql.stop().await?;
        assert_eq!(Status::Stopped, postgresql.status());

        Ok(())
    }

    #[test(tokio::test)]
    async fn test_unix_socket_with_builder() -> Result<()> {
        let socket_dir = tempfile::tempdir().expect("failed to create temp dir for socket");
        let socket_path = socket_dir.path().to_path_buf();

        let settings = SettingsBuilder::new()
            .socket_dir(socket_path.clone())
            .config("max_connections", "50")
            .build();

        assert_eq!(Some(socket_path), settings.socket_dir);
        assert_eq!(
            Some(&"50".to_string()),
            settings.configuration.get("max_connections")
        );

        let mut postgresql = PostgreSQL::new(settings);
        postgresql.setup().await?;
        postgresql.start().await?;

        let database_name = "builder_test";
        postgresql.create_database(database_name).await?;
        assert!(postgresql.database_exists(database_name).await?);
        postgresql.drop_database(database_name).await?;

        postgresql.stop().await?;
        Ok(())
    }

    #[test(tokio::test)]
    async fn test_unix_socket_temporary_cleanup() -> Result<()> {
        let socket_dir = tempfile::tempdir().expect("failed to create temp dir for socket");
        let socket_path = socket_dir.keep();

        let settings = SettingsBuilder::new()
            .socket_dir(socket_path.clone())
            .temporary(true)
            .build();
        let data_dir = settings.data_dir.clone();
        let password_file = settings.password_file.clone();

        {
            let mut postgresql = PostgreSQL::new(settings);
            postgresql.setup().await?;
            postgresql.start().await?;
            assert!(socket_path.exists());
        }

        // Verify that socket dir, data dir, and password file are cleaned up
        assert!(!data_dir.exists());
        assert!(!password_file.exists());
        assert!(!socket_path.exists());
        Ok(())
    }

    #[test]
    fn test_unix_socket_url_format() {
        let settings = SettingsBuilder::new()
            .host("localhost")
            .port(5432)
            .username("user")
            .password("pass")
            .socket_dir(PathBuf::from("/tmp/pg_socket"))
            .build();

        assert_eq!(
            "postgresql://user:pass@localhost:5432/test?host=%2Ftmp%2Fpg_socket",
            settings.url("test")
        );
    }

    #[test(tokio::test)]
    async fn test_connection_type_tcp_vs_unix_socket() -> Result<()> {
        let socket_dir = tempfile::tempdir().expect("failed to create temp dir for socket");
        let socket_path = socket_dir.path().to_path_buf();

        let settings = SettingsBuilder::new()
            .socket_dir(socket_path.clone())
            .build();

        let mut postgresql = PostgreSQL::new(settings);
        postgresql.setup().await?;
        postgresql.start().await?;

        let database_name = "conn_type_test";
        postgresql.create_database(database_name).await?;

        let settings = postgresql.settings();

        // Connect via TCP (construct URL without socket_dir query parameter)
        let tcp_url = format!(
            "postgresql://{}:{}@{}:{}/{}",
            settings.username, settings.password, settings.host, settings.port, database_name
        );
        let tcp_pool = PgPool::connect(tcp_url.as_str()).await.unwrap();
        let tcp_row = sqlx::query(
            "SELECT client_addr::TEXT, client_port \
             FROM pg_stat_activity \
             WHERE pid = pg_backend_pid()",
        )
        .fetch_one(&tcp_pool)
        .await
        .unwrap();
        let tcp_client_addr: Option<String> = tcp_row.get("client_addr");
        let tcp_client_port: Option<i32> = tcp_row.get("client_port");
        tcp_pool.close().await;

        // TCP connections have a non-null client_addr and a positive client_port
        assert!(
            tcp_client_addr.is_some(),
            "TCP connection should have a client_addr, got None"
        );
        assert!(
            tcp_client_port.is_some_and(|p| p > 0),
            "TCP connection should have a positive client_port, got {tcp_client_port:?}"
        );

        // Connect via Unix socket (URL includes ?host=<encoded_socket_dir>)
        let socket_url = settings.url(database_name);
        let socket_pool = PgPool::connect(socket_url.as_str()).await.unwrap();
        let socket_row = sqlx::query(
            "SELECT client_addr::TEXT, client_port \
             FROM pg_stat_activity \
             WHERE pid = pg_backend_pid()",
        )
        .fetch_one(&socket_pool)
        .await?;
        let socket_client_addr: Option<String> = socket_row.get("client_addr");
        let socket_client_port: Option<i32> = socket_row.get("client_port");
        socket_pool.close().await;

        // Unix socket connections have null client_addr and client_port of -1
        assert!(
            socket_client_addr.is_none(),
            "Unix socket connection should have null client_addr, got {socket_client_addr:?}"
        );
        assert_eq!(
            socket_client_port,
            Some(-1),
            "Unix socket connection should have client_port of -1, got {socket_client_port:?}"
        );

        postgresql.stop().await?;
        Ok(())
    }
}
