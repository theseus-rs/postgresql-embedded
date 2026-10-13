//! Shared helpers for command tests.
//!
//! Set `POSTGRESQL_COMMANDS_TEST_BIN` to a PostgreSQL 18 bin directory to run CLI checks.

use crate::initdb::InitDbBuilder;
use crate::pg_config::PgConfigBuilder;
use crate::postgres::PostgresBuilder;
use crate::{CommandBuilder, CommandExecutor};
use std::ffi::OsString;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tempfile::TempDir;

pub(crate) type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

pub(crate) fn assert_args<B: CommandBuilder + Clone>(builder: B, expected: &[&str]) {
    let expected: Vec<OsString> = expected.iter().map(OsString::from).collect();
    assert_eq!(builder.get_args(), expected);
    let command = builder.clone().build();
    assert_eq!(command.get_args().collect::<Vec<_>>(), expected);
    #[cfg(feature = "tokio")]
    {
        let command = builder.build_tokio();
        assert_eq!(command.as_std().get_args().collect::<Vec<_>>(), expected);
    }
}

pub(crate) fn binary_dir() -> Option<PathBuf> {
    std::env::var_os("POSTGRESQL_COMMANDS_TEST_BIN").map(PathBuf::from)
}

pub(crate) fn initialize_database(bin: &Path, builder: InitDbBuilder) -> TestResult<TempDir> {
    let temporary = tempfile::tempdir()?;
    let data = temporary.path().join("data");
    // pg_config resolves the share directory for both packaged and installed binaries.
    let (share, _) = PgConfigBuilder::new()
        .program_dir(bin)
        .sharedir()
        .build()
        .execute()?;
    builder
        .program_dir(bin)
        .pgdata(&data)
        .directory(share.trim())
        .no_locale()
        .auth("trust")
        .no_instructions()
        .build()
        .execute()?;
    Ok(temporary)
}

pub(crate) fn query_database(builder: PostgresBuilder, query: &[u8]) -> TestResult<String> {
    // Single-user mode needs no listener, port, or long-lived background server.
    let mut child = builder
        .single_user_mode()
        .dbname("postgres")
        .build()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(query)?;
    }
    let output = child.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stdout}\n{stderr}");
    Ok(stdout.into_owned())
}
