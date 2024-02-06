use crate::command::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// pg_dumpall extracts a PostgreSQL database cluster into an SQL script file.
#[derive(Clone, Debug, Default)]
pub struct PgDumpAllBuilder {
    program_dir: Option<PathBuf>,
    file: Option<OsString>,
    verbose: bool,
    version: bool,
    lock_wait_timeout: Option<u16>,
    help: bool,
    data_only: bool,
    clean: bool,
    encoding: Option<OsString>,
    globals_only: bool,
    no_owner: bool,
    roles_only: bool,
    schema_only: bool,
    superuser: Option<OsString>,
    tablespaces_only: bool,
    no_privileges: bool,
    binary_upgrade: bool,
    column_inserts: bool,
    disable_dollar_quoting: bool,
    disable_triggers: bool,
    exclude_database: Option<OsString>,
    extra_float_digits: Option<OsString>,
    if_exists: bool,
    inserts: bool,
    load_via_partition_root: bool,
    no_comments: bool,
    no_publications: bool,
    no_role_passwords: bool,
    no_security_labels: bool,
    no_subscriptions: bool,
    no_sync: bool,
    no_table_access_method: bool,
    no_tablespaces: bool,
    no_toast_compression: bool,
    no_unlogged_table_data: bool,
    on_conflict_do_nothing: bool,
    quote_all_identifiers: bool,
    rows_per_insert: Option<OsString>,
    use_set_session_authorization: bool,
    dbname: Option<OsString>,
    host: Option<OsString>,
    database: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
    role: Option<OsString>,
}

impl PgDumpAllBuilder {
    /// Create a new [`PgDumpAllBuilder`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Location of the program binary
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// output file name
    pub fn file<S: AsRef<OsStr>>(mut self, file: S) -> Self {
        self.file = Some(file.as_ref().to_os_string());
        self
    }

    /// verbose mode
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// output version information, then exit
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// fail after waiting TIMEOUT for a table lock
    pub fn lock_wait_timeout(mut self, lock_wait_timeout: u16) -> Self {
        self.lock_wait_timeout = Some(lock_wait_timeout);
        self
    }

    /// show help, then exit
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }
}

impl CommandBuilder for PgDumpAllBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_dumpall".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(file) = &self.file {
            args.push("--file".into());
            args.push(file.into());
        }

        if self.verbose {
            args.push("--verbose".into());
        }

        if self.version {
            args.push("--version".into());
        }

        if let Some(lock_wait_timeout) = &self.lock_wait_timeout {
            args.push("--lock-wait-timeout".into());
            args.push(lock_wait_timeout.to_string().into());
        }

        if self.help {
            args.push("--help".into());
        }

        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::traits::CommandToString;

    #[test]
    fn test_builder_new() {
        let command = PgDumpAllBuilder::new().program_dir(".").build();

        assert_eq!(
            PathBuf::from(".").join("pg_dumpall"),
            PathBuf::from(command.to_command_string().replace("\"", ""))
        );
    }

    #[test]
    fn test_builder() {
        let command = PgDumpAllBuilder::new()
            .file("dump.sql")
            .verbose()
            .version()
            .lock_wait_timeout(10)
            .help()
            .build();

        assert_eq!(
            r#""pg_dumpall" "--file" "dump.sql" "--verbose" "--version" "--lock-wait-timeout" "10" "--help""#,
            command.to_command_string()
        );
    }
}
