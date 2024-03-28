use crate::command::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// pg_dump dumps a database as a text file or to other formats.
#[derive(Clone, Debug, Default)]
pub struct PgDumpBuilder {
    program_dir: Option<PathBuf>,
    file: Option<OsString>,
    format: Option<OsString>,
    jobs: Option<OsString>,
    verbose: bool,
    version: bool,
    compress: Option<OsString>,
    lock_wait_timeout: Option<u16>,
    no_sync: bool,
    help: bool,
    host: Option<OsString>,
    port: Option<u16>,
    dbname: Option<OsString>,
    username: Option<OsString>,
    pg_password: Option<OsString>,
    schema_only: bool,
    no_password: bool,
    password: bool,
}

impl PgDumpBuilder {
    /// Create a new [`PgDumpBuilder`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Location of the program binary
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// Set the output file or directory name
    pub fn file<S: AsRef<OsStr>>(mut self, file: S) -> Self {
        self.file = Some(file.as_ref().to_os_string());
        self
    }

    /// Set the output file format (custom, directory, tar, plain text (default))
    pub fn format<S: AsRef<OsStr>>(mut self, format: S) -> Self {
        self.format = Some(format.as_ref().to_os_string());
        self
    }

    /// Use this many parallel jobs to dump
    pub fn jobs<S: AsRef<OsStr>>(mut self, jobs: S) -> Self {
        self.jobs = Some(jobs.as_ref().to_os_string());
        self
    }

    /// Enable verbose mode
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// Output version information, then exit
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// Compress as specified
    pub fn compress<S: AsRef<OsStr>>(mut self, compress: S) -> Self {
        self.compress = Some(compress.as_ref().to_os_string());
        self
    }

    /// Fail after waiting TIMEOUT for a table lock
    pub fn lock_wait_timeout(mut self, lock_wait_timeout: u16) -> Self {
        self.lock_wait_timeout = Some(lock_wait_timeout);
        self
    }

    /// Do not wait for changes to be written safely to disk
    pub fn no_sync(mut self) -> Self {
        self.no_sync = true;
        self
    }

    /// Show help, then exit
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }

    /// database name to dump
    pub fn dbname<S: AsRef<OsStr>>(mut self, dbname: S) -> Self {
        self.dbname = Some(dbname.as_ref().to_os_string());
        self
    }

    /// database server host or socket directory
    pub fn host<S: AsRef<OsStr>>(mut self, host: S) -> Self {
        self.host = Some(host.as_ref().to_os_string());
        self
    }

    /// database server port
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// database user name
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
        self
    }

    /// user password
    pub fn pg_password<S: AsRef<OsStr>>(mut self, pg_password: S) -> Self {
        self.pg_password = Some(pg_password.as_ref().to_os_string());
        self
    }

    /// Dumps only the schema
    pub fn schema_only(mut self) -> Self {
        self.schema_only = true;
        self
    }

    /// never prompt for password
    pub fn no_password(mut self) -> Self {
        self.no_password = true;
        self
    }
}

impl CommandBuilder for PgDumpBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_dump".as_ref()
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

        if let Some(format) = &self.format {
            args.push("--format".into());
            args.push(format.into());
        }

        if let Some(jobs) = &self.jobs {
            args.push("--jobs".into());
            args.push(jobs.into());
        }

        if self.verbose {
            args.push("--verbose".into());
        }

        if self.version {
            args.push("--version".into());
        }

        if let Some(compress) = &self.compress {
            args.push("--compress".into());
            args.push(compress.into());
        }

        if let Some(lock_wait_timeout) = &self.lock_wait_timeout {
            args.push("--lock-wait-timeout".into());
            args.push(lock_wait_timeout.to_string().into());
        }

        if self.no_sync {
            args.push("--no-sync".into());
        }

        if self.help {
            args.push("--help".into());
        }

        if self.schema_only {
            args.push("--schema-only".into());
        }

        if let Some(dbname) = &self.dbname {
            args.push("--dbname".into());
            args.push(dbname.into());
        }

        if let Some(host) = &self.host {
            args.push("--host".into());
            args.push(host.into());
        }

        if let Some(port) = &self.port {
            args.push("--port".into());
            args.push(port.to_string().into());
        }

        if let Some(username) = &self.username {
            args.push("--username".into());
            args.push(username.into());
        }

        if self.no_password {
            args.push("--no-password".into());
        }

        if self.password {
            args.push("--password".into());
        }

        args
    }

    /// Get the environment variables for the command
    fn get_envs(&self) -> Vec<(OsString, OsString)> {
        let mut envs: Vec<(OsString, OsString)> = Vec::new();

        if let Some(password) = &self.pg_password {
            envs.push(("PGPASSWORD".into(), password.into()));
        }

        envs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::traits::CommandToString;
    use test_log::test;

    #[test]
    fn test_builder_new() {
        let command = PgDumpBuilder::new().program_dir(".").build();

        assert_eq!(
            PathBuf::from(".").join("pg_dump"),
            PathBuf::from(command.to_command_string().replace("\"", ""))
        );
    }

    #[test]
    fn test_builder() {
        let command = PgDumpBuilder::new()
            .file("file")
            .format("format")
            .jobs("jobs")
            .verbose()
            .version()
            .compress("compress")
            .lock_wait_timeout(10)
            .no_sync()
            .help()
            .build();
        assert_eq!(
            r#""pg_dump" "--file" "file" "--format" "format" "--jobs" "jobs" "--verbose" "--version" "--compress" "compress" "--lock-wait-timeout" "10" "--no-sync" "--help""#,
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder_schema_only() {
        let command = PgDumpBuilder::new()
            .file("file")
            .schema_only()
            .dbname("dbname")
            .host("host")
            .port(123)
            .username("username")
            .pg_password("password")
            .no_password()
            .build();
        assert_eq!(
            r#"PGPASSWORD="password" "pg_dump" "--file" "file" "--schema-only" "--dbname" "dbname" "--host" "host" "--port" "123" "--username" "username" "--no-password""#,
            command.to_command_string()
        );
    }
}
