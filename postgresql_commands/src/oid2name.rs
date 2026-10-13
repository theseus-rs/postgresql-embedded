use crate::Settings;
use crate::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `oid2name` helps to examine the file structure used by `PostgreSQL`.
#[derive(Clone, Debug, Default)]
pub struct Oid2NameBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    filenode: Vec<OsString>,
    indexes: bool,
    oid: Vec<OsString>,
    quiet: bool,
    tablespaces: bool,
    system_objects: bool,
    table: Vec<OsString>,
    version: bool,
    extended: bool,
    help: bool,
    dbname: Option<OsString>,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
}

impl Oid2NameBuilder {
    /// Create a new [`Oid2NameBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`Oid2NameBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        let mut builder = Self::new()
            .program_dir(settings.get_binary_dir())
            .host(settings.get_host())
            .port(settings.get_port())
            .username(settings.get_username());
        if let Some(socket_dir) = settings.get_socket_dir() {
            builder = builder.host(socket_dir.to_string_lossy().to_string());
        }
        builder
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// show info for table with given file node
    /// May be called multiple times; each value is retained.
    #[must_use]
    pub fn filenode<S: AsRef<OsStr>>(mut self, filenode: S) -> Self {
        self.filenode.push(filenode.as_ref().to_os_string());
        self
    }

    /// show indexes and sequences too
    #[must_use]
    pub fn indexes(mut self) -> Self {
        self.indexes = true;
        self
    }

    /// show info for table with given OID
    /// May be called multiple times; each value is retained.
    #[must_use]
    pub fn oid<S: AsRef<OsStr>>(mut self, oid: S) -> Self {
        self.oid.push(oid.as_ref().to_os_string());
        self
    }

    /// quiet (don't show headers)
    #[must_use]
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// show all tablespaces
    #[must_use]
    pub fn tablespaces(mut self) -> Self {
        self.tablespaces = true;
        self
    }

    /// show system objects too
    #[must_use]
    pub fn system_objects(mut self) -> Self {
        self.system_objects = true;
        self
    }

    /// show info for named table
    /// May be called multiple times; each value is retained.
    #[must_use]
    pub fn table<S: AsRef<OsStr>>(mut self, table: S) -> Self {
        self.table.push(table.as_ref().to_os_string());
        self
    }

    /// output version information, then exit
    #[must_use]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// extended (show additional columns)
    #[must_use]
    pub fn extended(mut self) -> Self {
        self.extended = true;
        self
    }

    /// show help, then exit
    #[must_use]
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }

    /// database to connect to
    #[must_use]
    pub fn dbname<S: AsRef<OsStr>>(mut self, dbname: S) -> Self {
        self.dbname = Some(dbname.as_ref().to_os_string());
        self
    }

    /// database server host or socket directory
    #[must_use]
    pub fn host<S: AsRef<OsStr>>(mut self, host: S) -> Self {
        self.host = Some(host.as_ref().to_os_string());
        self
    }

    /// database server port number
    #[must_use]
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// connect as specified database user
    #[must_use]
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
        self
    }
}

impl CommandBuilder for Oid2NameBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "oid2name".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if self.version {
            args.push("--version".into());
        }

        if self.help {
            args.push("--help".into());
        }

        for filenode in &self.filenode {
            args.push("--filenode".into());
            args.push(filenode.into());
        }

        if self.indexes {
            args.push("--indexes".into());
        }

        for oid in &self.oid {
            args.push("--oid".into());
            args.push(oid.into());
        }

        if self.quiet {
            args.push("--quiet".into());
        }

        if self.tablespaces {
            args.push("--tablespaces".into());
        }

        if self.system_objects {
            args.push("--system-objects".into());
        }

        for table in &self.table {
            args.push("--table".into());
            args.push(table.into());
        }

        if self.extended {
            args.push("--extended".into());
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

        args
    }

    /// Get the environment variables for the command
    fn get_envs(&self) -> Vec<(OsString, OsString)> {
        self.envs.clone()
    }

    /// Set an environment variable for the command
    fn env<S: AsRef<OsStr>>(mut self, key: S, value: S) -> Self {
        self.envs
            .push((key.as_ref().to_os_string(), value.as_ref().to_os_string()));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CommandExecutor;
    use crate::TestSettings;
    use crate::TestSocketSettings;
    use crate::test_utils::{TestResult, assert_args, binary_dir};
    use crate::traits::CommandToString;
    use test_log::test;

    #[test]
    fn test_builder_new() {
        let command = Oid2NameBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("oid2name"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = Oid2NameBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#""./oid2name" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\oid2name" "#;

        assert_eq!(
            format!(
                r#"{command_prefix}"--host" "localhost" "--port" "5432" "--username" "postgres""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder() {
        let command = Oid2NameBuilder::new()
            .env("PGDATABASE", "database")
            .filenode("filenode")
            .indexes()
            .oid("oid")
            .quiet()
            .tablespaces()
            .system_objects()
            .table("table")
            .version()
            .extended()
            .help()
            .dbname("dbname")
            .host("localhost")
            .port(5432)
            .username("username")
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"oid2name" "--version" "--help" "--filenode" "filenode" "--indexes" "--oid" "oid" "--quiet" "--tablespaces" "--system-objects" "--table" "table" "--extended" "--dbname" "dbname" "--host" "localhost" "--port" "5432" "--username" "username""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder_from_socket() {
        let command = Oid2NameBuilder::from(&TestSocketSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#""./oid2name" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\oid2name" "#;

        assert_eq!(
            format!(
                r#"{command_prefix}"--host" "/tmp/pg_socket" "--port" "5432" "--username" "postgres""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_repeated_options() {
        assert_args(
            Oid2NameBuilder::new().filenode("first").filenode("second"),
            &["--filenode", "first", "--filenode", "second"],
        );
        assert_args(
            Oid2NameBuilder::new().oid("first").oid("second"),
            &["--oid", "first", "--oid", "second"],
        );
        assert_args(
            Oid2NameBuilder::new().table("first").table("second"),
            &["--table", "first", "--table", "second"],
        );
    }

    #[test]
    fn test_informational_options_come_first_on_configured_builders() {
        let version = Oid2NameBuilder::from(&TestSettings).version().get_args();
        assert_eq!(version.first(), Some(&OsString::from("--version")));
        let help = Oid2NameBuilder::from(&TestSettings).help().get_args();
        assert_eq!(help.first(), Some(&OsString::from("--help")));
    }

    #[test]
    fn test_cli_help_and_version() -> TestResult {
        let Some(bin) = binary_dir() else {
            return Ok(());
        };
        Oid2NameBuilder::new()
            .program_dir(&bin)
            .version()
            .build()
            .execute()?;
        Oid2NameBuilder::new()
            .program_dir(&bin)
            .help()
            .build()
            .execute()?;
        Ok(())
    }
}
