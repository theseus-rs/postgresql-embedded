use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_isready` issues a connection check to a PostgreSQL database.
#[derive(Clone, Debug, Default)]
pub struct PgIsReadyBuilder {
    program_dir: Option<PathBuf>,
    dbname: Option<OsString>,
    quiet: bool,
    version: bool,
    help: bool,
    host: Option<OsString>,
    port: Option<u16>,
    timeout: Option<u16>,
    username: Option<OsString>,
}

impl PgIsReadyBuilder {
    /// Create a new [PgIsReadyBuilder]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [PgIsReadyBuilder] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new()
            .program_dir(settings.get_binary_dir())
            .host(settings.get_host())
            .port(settings.get_port())
            .username(settings.get_username())
    }

    /// Location of the program binary
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// Set the database name
    pub fn dbname<S: AsRef<OsStr>>(mut self, dbname: S) -> Self {
        self.dbname = Some(dbname.as_ref().to_os_string());
        self
    }

    /// Run quietly
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// Output version information, then exit
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// Show help, then exit
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }

    /// Set the database server host or socket directory
    pub fn host<S: AsRef<OsStr>>(mut self, host: S) -> Self {
        self.host = Some(host.as_ref().to_os_string());
        self
    }

    /// Set the database server port
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Set the seconds to wait when attempting connection, 0 disables (default: 3)
    pub fn timeout(mut self, timeout: u16) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the user name to connect as
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
        self
    }
}

impl CommandBuilder for PgIsReadyBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_isready".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(dbname) = &self.dbname {
            args.push("--dbname".into());
            args.push(dbname.into());
        }

        if self.quiet {
            args.push("--quiet".into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.help {
            args.push("--help".into());
        }

        if let Some(host) = &self.host {
            args.push("--host".into());
            args.push(host.into());
        }

        if let Some(port) = &self.port {
            args.push("--port".into());
            args.push(port.to_string().into());
        }

        if let Some(timeout) = &self.timeout {
            args.push("--timeout".into());
            args.push(timeout.to_string().into());
        }

        if let Some(username) = &self.username {
            args.push("--username".into());
            args.push(username.into());
        }

        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::CommandToString;
    use crate::TestSettings;
    use test_log::test;

    #[test]
    fn test_builder_new() {
        let command = PgIsReadyBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_isready"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgIsReadyBuilder::from(&TestSettings).build();
        assert_eq!(
            r#""./pg_isready" "--host" "localhost" "--port" "5432" "--username" "postgres""#,
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder() {
        let command = PgIsReadyBuilder::new()
            .dbname("postgres")
            .quiet()
            .version()
            .help()
            .host("localhost")
            .port(5432)
            .timeout(3)
            .username("postgres")
            .build();

        assert_eq!(
            r#""pg_isready" "--dbname" "postgres" "--quiet" "--version" "--help" "--host" "localhost" "--port" "5432" "--timeout" "3" "--username" "postgres""#,
            command.to_command_string()
        );
    }
}
