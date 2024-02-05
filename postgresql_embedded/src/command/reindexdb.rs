use crate::command::traits::CommandBuilder;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// reindexdb reindexes a PostgreSQL database.
#[derive(Clone, Debug, Default)]
pub struct ReindexDbBuilder {
    program_dir: Option<PathBuf>,
    all: bool,
    concurrently: bool,
    dbname: Option<OsString>,
    echo: bool,
    index: Option<OsString>,
    jobs: Option<u32>,
    quiet: bool,
    system: bool,
    schema: Option<OsString>,
    table: Option<OsString>,
    tablespace: Option<OsString>,
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

impl ReindexDbBuilder {
    /// Create a new [`ReindexDbBuilder`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Location of the program binary
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// reindex all databases
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// reindex concurrently
    pub fn concurrently(mut self) -> Self {
        self.concurrently = true;
        self
    }

    /// database to reindex
    pub fn dbname<S: AsRef<OsStr>>(mut self, dbname: S) -> Self {
        self.dbname = Some(dbname.as_ref().to_os_string());
        self
    }

    /// show the commands being sent to the server
    pub fn echo(mut self) -> Self {
        self.echo = true;
        self
    }

    /// recreate specific index(es) only
    pub fn index<S: AsRef<OsStr>>(mut self, index: S) -> Self {
        self.index = Some(index.as_ref().to_os_string());
        self
    }

    /// use this many concurrent connections to reindex
    pub fn jobs(mut self, jobs: u32) -> Self {
        self.jobs = Some(jobs);
        self
    }

    /// don't write any messages
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// reindex system catalogs only
    pub fn system(mut self) -> Self {
        self.system = true;
        self
    }

    /// reindex specific schema(s) only
    pub fn schema<S: AsRef<OsStr>>(mut self, schema: S) -> Self {
        self.schema = Some(schema.as_ref().to_os_string());
        self
    }

    /// reindex specific table(s) only
    pub fn table<S: AsRef<OsStr>>(mut self, table: S) -> Self {
        self.table = Some(table.as_ref().to_os_string());
        self
    }

    /// tablespace where indexes are rebuilt
    pub fn tablespace<S: AsRef<OsStr>>(mut self, tablespace: S) -> Self {
        self.tablespace = Some(tablespace.as_ref().to_os_string());
        self
    }

    /// write a lot of output
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// output version information, then exit
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// show help, then exit
    pub fn help(mut self) -> Self {
        self.help = true;
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

    /// user name to connect as
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
        self
    }

    /// never prompt for password
    pub fn no_password(mut self) -> Self {
        self.no_password = true;
        self
    }

    /// force password prompt
    pub fn password(mut self) -> Self {
        self.password = true;
        self
    }

    /// alternate maintenance database
    pub fn maintenance_db<S: AsRef<OsStr>>(mut self, maintenance_db: S) -> Self {
        self.maintenance_db = Some(maintenance_db.as_ref().to_os_string());
        self
    }
}

impl CommandBuilder for ReindexDbBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "reindexdb".as_ref()
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

        if self.concurrently {
            args.push("--concurrently".into());
        }

        if let Some(dbname) = &self.dbname {
            args.push("--dbname".into());
            args.push(dbname.into());
        }

        if self.echo {
            args.push("--echo".into());
        }

        if let Some(index) = &self.index {
            args.push("--index".into());
            args.push(index.into());
        }

        if let Some(jobs) = &self.jobs {
            args.push("--jobs".into());
            args.push(jobs.to_string().into());
        }

        if self.quiet {
            args.push("--quiet".into());
        }

        if self.system {
            args.push("--system".into());
        }

        if let Some(schema) = &self.schema {
            args.push("--schema".into());
            args.push(schema.into());
        }

        if let Some(table) = &self.table {
            args.push("--table".into());
            args.push(table.into());
        }

        if let Some(tablespace) = &self.tablespace {
            args.push("--tablespace".into());
            args.push(tablespace.into());
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
        let command = ReindexDbBuilder::new().build();

        assert_eq!(r#""reindexdb""#, command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = ReindexDbBuilder::new()
            .program_dir("/usr/bin")
            .all()
            .concurrently()
            .dbname("dbname")
            .echo()
            .index("index")
            .jobs(1)
            .quiet()
            .system()
            .schema("schema")
            .table("table")
            .tablespace("tablespace")
            .verbose()
            .version()
            .help()
            .host("localhost")
            .port(5432)
            .username("username")
            .no_password()
            .password()
            .maintenance_db("maintenance-db")
            .build();

        assert_eq!(
            r#""/usr/bin/reindexdb" "--all" "--concurrently" "--dbname" "dbname" "--echo" "--index" "index" "--jobs" "1" "--quiet" "--system" "--schema" "schema" "--table" "table" "--tablespace" "tablespace" "--verbose" "--version" "--help" "--host" "localhost" "--port" "5432" "--username" "username" "--no-password" "--password" "--maintenance-db" "maintenance-db""#,
            command.to_command_string()
        );
    }
}
