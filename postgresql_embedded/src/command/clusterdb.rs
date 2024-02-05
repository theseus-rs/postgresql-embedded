use crate::command::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// clusterdb clusters all previously clustered tables in a database.
#[derive(Clone, Debug, Default)]
pub struct ClusterDbBuilder {
    program_dir: Option<PathBuf>,
    all: bool,
    dbname: Option<OsString>,
    echo: bool,
    quiet: bool,
    table: Option<OsString>,
    verbose: bool,
    version: bool,
    help: bool,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
    maintenance_db: Option<OsString>,
}

impl ClusterDbBuilder {
    /// Create a new [`ClusterDbBuilder`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Location of the program binary
    fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// Cluster all databases
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// Database to cluster
    pub fn dbname<S: AsRef<OsStr>>(mut self, dbname: S) -> Self {
        self.dbname = Some(dbname.as_ref().to_os_string());
        self
    }

    /// Show the commands being sent to the server
    pub fn echo(mut self) -> Self {
        self.echo = true;
        self
    }

    /// Don't write any messages
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// Cluster specific table(s) only
    pub fn table<S: AsRef<OsStr>>(mut self, table: S) -> Self {
        self.table = Some(table.as_ref().to_os_string());
        self
    }

    /// Write a lot of output
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
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

    /// Database server host or socket directory
    pub fn host<S: AsRef<OsStr>>(mut self, host: S) -> Self {
        self.host = Some(host.as_ref().to_os_string());
        self
    }

    /// Database server port
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// User name to connect as
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
        self
    }

    /// Never prompt for password
    pub fn no_password(mut self) -> Self {
        self.no_password = true;
        self
    }

    /// Force password prompt
    pub fn password(mut self) -> Self {
        self.password = true;
        self
    }

    /// Alternate maintenance database
    pub fn maintenance_db<S: AsRef<OsStr>>(mut self, db: S) -> Self {
        self.maintenance_db = Some(db.as_ref().to_os_string());
        self
    }
}

impl CommandBuilder for ClusterDbBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "clusterdb".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if self.all {
            args.push("--all".into());
        }

        if let Some(dbname) = &self.dbname {
            args.push("--dbname".into());
            args.push(dbname.into());
        }

        if self.echo {
            args.push("--echo".into());
        }

        if self.quiet {
            args.push("--quiet".into());
        }

        if let Some(table) = &self.table {
            args.push("--table".into());
            args.push(table.into());
        }

        if self.verbose {
            args.push("--verbose".into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.help {
            args.push("--help".into());
        }

        if let Some(host) = &self.host {
            args.push("--host".into());
            args.push(host.into())
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

        if let Some(maintenance_db) = &self.maintenance_db {
            args.push("--maintenance-db".into());
            args.push(maintenance_db.into());
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
        let command = ClusterDbBuilder::new().build();

        assert_eq!(r#""clusterdb""#, command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = ClusterDbBuilder::new()
            .program_dir("/usr/bin")
            .all()
            .dbname("test")
            .echo()
            .quiet()
            .table("table")
            .verbose()
            .version()
            .help()
            .host("localhost")
            .port(5432)
            .username("postgres")
            .no_password()
            .password()
            .maintenance_db("postgres")
            .build();

        assert_eq!(
            r#""/usr/bin/clusterdb" "--all" "--dbname" "test" "--echo" "--quiet" "--table" "table" "--verbose" "--version" "--help" "--host" "localhost" "--port" "5432" "--username" "postgres" "--no-password" "--password" "--maintenance-db" "postgres""#,
            command.to_command_string()
        );
    }
}
