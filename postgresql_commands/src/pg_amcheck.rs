use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_amcheck` checks objects in a `PostgreSQL` database for corruption.
#[derive(Clone, Debug, Default)]
pub struct PgAmCheckBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    all: bool,
    database: Option<OsString>,
    exclude_database: Option<OsString>,
    index: Option<OsString>,
    exclude_index: Option<OsString>,
    relation: Option<OsString>,
    exclude_relation: Option<OsString>,
    schema: Option<OsString>,
    exclude_schema: Option<OsString>,
    table: Option<OsString>,
    exclude_table: Option<OsString>,
    no_dependent_indexes: bool,
    no_dependent_toast: bool,
    no_strict_names: bool,
    exclude_toast_pointers: bool,
    on_error_stop: bool,
    skip: Option<OsString>,
    start_block: Option<OsString>,
    end_block: Option<OsString>,
    heap_all_indexed: bool,
    parent_check: bool,
    root_descend: bool,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
    pg_password: Option<OsString>,
    maintenance_db: Option<OsString>,
    echo: bool,
    jobs: Option<OsString>,
    progress: bool,
    verbose: bool,
    version: bool,
    install_missing: bool,
    help: bool,
}

impl PgAmCheckBuilder {
    /// Create a new [`PgAmCheckBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgAmCheckBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new()
            .program_dir(settings.get_binary_dir())
            .host(settings.get_host())
            .port(settings.get_port())
            .username(settings.get_username())
            .pg_password(settings.get_password())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// check all databases
    #[must_use]
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// check matching database(s)
    #[must_use]
    pub fn database<S: AsRef<OsStr>>(mut self, database: S) -> Self {
        self.database = Some(database.as_ref().to_os_string());
        self
    }

    /// do NOT check matching database(s)
    #[must_use]
    pub fn exclude_database<S: AsRef<OsStr>>(mut self, exclude_database: S) -> Self {
        self.exclude_database = Some(exclude_database.as_ref().to_os_string());
        self
    }

    /// check matching index(es)
    #[must_use]
    pub fn index<S: AsRef<OsStr>>(mut self, index: S) -> Self {
        self.index = Some(index.as_ref().to_os_string());
        self
    }

    /// do NOT check matching index(es)
    #[must_use]
    pub fn exclude_index<S: AsRef<OsStr>>(mut self, exclude_index: S) -> Self {
        self.exclude_index = Some(exclude_index.as_ref().to_os_string());
        self
    }

    /// check matching relation(s)
    #[must_use]
    pub fn relation<S: AsRef<OsStr>>(mut self, relation: S) -> Self {
        self.relation = Some(relation.as_ref().to_os_string());
        self
    }

    /// do NOT check matching relation(s)
    #[must_use]
    pub fn exclude_relation<S: AsRef<OsStr>>(mut self, exclude_relation: S) -> Self {
        self.exclude_relation = Some(exclude_relation.as_ref().to_os_string());
        self
    }

    /// check matching schema(s)
    #[must_use]
    pub fn schema<S: AsRef<OsStr>>(mut self, schema: S) -> Self {
        self.schema = Some(schema.as_ref().to_os_string());
        self
    }

    /// do NOT check matching schema(s)
    #[must_use]
    pub fn exclude_schema<S: AsRef<OsStr>>(mut self, exclude_schema: S) -> Self {
        self.exclude_schema = Some(exclude_schema.as_ref().to_os_string());
        self
    }

    /// check matching table(s)
    #[must_use]
    pub fn table<S: AsRef<OsStr>>(mut self, table: S) -> Self {
        self.table = Some(table.as_ref().to_os_string());
        self
    }

    /// do NOT check matching table(s)
    #[must_use]
    pub fn exclude_table<S: AsRef<OsStr>>(mut self, exclude_table: S) -> Self {
        self.exclude_table = Some(exclude_table.as_ref().to_os_string());
        self
    }

    /// do NOT expand list of relations to include indexes
    #[must_use]
    pub fn no_dependent_indexes(mut self) -> Self {
        self.no_dependent_indexes = true;
        self
    }

    /// do NOT expand list of relations to include TOAST tables
    #[must_use]
    pub fn no_dependent_toast(mut self) -> Self {
        self.no_dependent_toast = true;
        self
    }

    /// do NOT require patterns to match objects
    #[must_use]
    pub fn no_strict_names(mut self) -> Self {
        self.no_strict_names = true;
        self
    }

    /// do NOT follow relation TOAST pointers
    #[must_use]
    pub fn exclude_toast_pointers(mut self) -> Self {
        self.exclude_toast_pointers = true;
        self
    }

    /// stop checking at end of first corrupt page
    #[must_use]
    pub fn on_error_stop(mut self) -> Self {
        self.on_error_stop = true;
        self
    }

    /// do NOT check "all-frozen" or "all-visible" blocks
    #[must_use]
    pub fn skip<S: AsRef<OsStr>>(mut self, skip: S) -> Self {
        self.skip = Some(skip.as_ref().to_os_string());
        self
    }

    /// begin checking table(s) at the given block number
    #[must_use]
    pub fn start_block<S: AsRef<OsStr>>(mut self, start_block: S) -> Self {
        self.start_block = Some(start_block.as_ref().to_os_string());
        self
    }

    /// check table(s) only up to the given block number
    #[must_use]
    pub fn end_block<S: AsRef<OsStr>>(mut self, end_block: S) -> Self {
        self.end_block = Some(end_block.as_ref().to_os_string());
        self
    }

    /// check that all heap tuples are found within indexes
    #[must_use]
    pub fn heap_all_indexed(mut self) -> Self {
        self.heap_all_indexed = true;
        self
    }

    /// check index parent/child relationships
    #[must_use]
    pub fn parent_check(mut self) -> Self {
        self.parent_check = true;
        self
    }

    /// search from root page to refind tuples
    #[must_use]
    pub fn root_descend(mut self) -> Self {
        self.root_descend = true;
        self
    }

    /// database server host or socket directory
    #[must_use]
    pub fn host<S: AsRef<OsStr>>(mut self, host: S) -> Self {
        self.host = Some(host.as_ref().to_os_string());
        self
    }

    /// database server port
    #[must_use]
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// user name to connect as
    #[must_use]
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
        self
    }

    /// never prompt for password
    #[must_use]
    pub fn no_password(mut self) -> Self {
        self.no_password = true;
        self
    }

    /// force password prompt
    #[must_use]
    pub fn password(mut self) -> Self {
        self.password = true;
        self
    }

    /// user password
    #[must_use]
    pub fn pg_password<S: AsRef<OsStr>>(mut self, pg_password: S) -> Self {
        self.pg_password = Some(pg_password.as_ref().to_os_string());
        self
    }

    /// alternate maintenance database
    #[must_use]
    pub fn maintenance_db<S: AsRef<OsStr>>(mut self, maintenance_db: S) -> Self {
        self.maintenance_db = Some(maintenance_db.as_ref().to_os_string());
        self
    }

    /// show the commands being sent to the server
    #[must_use]
    pub fn echo(mut self) -> Self {
        self.echo = true;
        self
    }

    /// use this many concurrent connections to the server
    #[must_use]
    pub fn jobs<S: AsRef<OsStr>>(mut self, jobs: S) -> Self {
        self.jobs = Some(jobs.as_ref().to_os_string());
        self
    }

    /// show progress information
    #[must_use]
    pub fn progress(mut self) -> Self {
        self.progress = true;
        self
    }

    /// write a lot of output
    #[must_use]
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// output version information, then exit
    #[must_use]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// install missing extensions
    #[must_use]
    pub fn install_missing(mut self) -> Self {
        self.install_missing = true;
        self
    }

    /// show help, then exit
    #[must_use]
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }
}

impl CommandBuilder for PgAmCheckBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_amcheck".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    #[expect(clippy::too_many_lines)]
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if self.all {
            args.push("--all".into());
        }

        if let Some(database) = &self.database {
            args.push("--database".into());
            args.push(database.into());
        }

        if let Some(exclude_database) = &self.exclude_database {
            args.push("--exclude-database".into());
            args.push(exclude_database.into());
        }

        if let Some(index) = &self.index {
            args.push("--index".into());
            args.push(index.into());
        }

        if let Some(exclude_index) = &self.exclude_index {
            args.push("--exclude-index".into());
            args.push(exclude_index.into());
        }

        if let Some(relation) = &self.relation {
            args.push("--relation".into());
            args.push(relation.into());
        }

        if let Some(exclude_relation) = &self.exclude_relation {
            args.push("--exclude-relation".into());
            args.push(exclude_relation.into());
        }

        if let Some(schema) = &self.schema {
            args.push("--schema".into());
            args.push(schema.into());
        }

        if let Some(exclude_schema) = &self.exclude_schema {
            args.push("--exclude-schema".into());
            args.push(exclude_schema.into());
        }

        if let Some(table) = &self.table {
            args.push("--table".into());
            args.push(table.into());
        }

        if let Some(exclude_table) = &self.exclude_table {
            args.push("--exclude-table".into());
            args.push(exclude_table.into());
        }

        if self.no_dependent_indexes {
            args.push("--no-dependent-indexes".into());
        }

        if self.no_dependent_toast {
            args.push("--no-dependent-toast".into());
        }

        if self.no_strict_names {
            args.push("--no-strict-names".into());
        }

        if self.exclude_toast_pointers {
            args.push("--exclude-toast-pointers".into());
        }

        if self.on_error_stop {
            args.push("--on-error-stop".into());
        }

        if let Some(skip) = &self.skip {
            args.push("--skip".into());
            args.push(skip.into());
        }

        if let Some(start_block) = &self.start_block {
            args.push("--startblock".into());
            args.push(start_block.into());
        }

        if let Some(end_block) = &self.end_block {
            args.push("--endblock".into());
            args.push(end_block.into());
        }

        if self.heap_all_indexed {
            args.push("--heapallindexed".into());
        }

        if self.parent_check {
            args.push("--parent-check".into());
        }

        if self.root_descend {
            args.push("--rootdescend".into());
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

        if self.echo {
            args.push("--echo".into());
        }

        if let Some(jobs) = &self.jobs {
            args.push("--jobs".into());
            args.push(jobs.into());
        }

        if self.progress {
            args.push("--progress".into());
        }

        if self.verbose {
            args.push("--verbose".into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.install_missing {
            args.push("--install-missing".into());
        }

        if self.help {
            args.push("--help".into());
        }

        args
    }

    /// Get the environment variables for the command
    fn get_envs(&self) -> Vec<(OsString, OsString)> {
        let mut envs: Vec<(OsString, OsString)> = self.envs.clone();

        if let Some(password) = &self.pg_password {
            envs.push(("PGPASSWORD".into(), password.into()));
        }

        envs
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
    use crate::traits::CommandToString;
    use crate::TestSettings;
    use test_log::test;

    #[test]
    fn test_builder_new() {
        let command = PgAmCheckBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_amcheck"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgAmCheckBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGPASSWORD="password" "./pg_amcheck" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\pg_amcheck" "#;

        assert_eq!(
            format!(
                r#"{command_prefix}"--host" "localhost" "--port" "5432" "--username" "postgres""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder() {
        let command = PgAmCheckBuilder::new()
            .env("PGDATABASE", "database")
            .all()
            .database("database")
            .exclude_database("exclude_database")
            .index("index")
            .exclude_index("exclude_index")
            .relation("relation")
            .exclude_relation("exclude_relation")
            .schema("schema")
            .exclude_schema("exclude_schema")
            .table("table")
            .exclude_table("exclude_table")
            .no_dependent_indexes()
            .no_dependent_toast()
            .no_strict_names()
            .exclude_toast_pointers()
            .on_error_stop()
            .skip("skip")
            .start_block("start_block")
            .end_block("end_block")
            .heap_all_indexed()
            .parent_check()
            .root_descend()
            .host("localhost")
            .port(5432)
            .username("username")
            .no_password()
            .password()
            .pg_password("password")
            .maintenance_db("maintenance_db")
            .echo()
            .jobs("jobs")
            .progress()
            .verbose()
            .version()
            .install_missing()
            .help()
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" PGPASSWORD="password" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"pg_amcheck" "--all" "--database" "database" "--exclude-database" "exclude_database" "--index" "index" "--exclude-index" "exclude_index" "--relation" "relation" "--exclude-relation" "exclude_relation" "--schema" "schema" "--exclude-schema" "exclude_schema" "--table" "table" "--exclude-table" "exclude_table" "--no-dependent-indexes" "--no-dependent-toast" "--no-strict-names" "--exclude-toast-pointers" "--on-error-stop" "--skip" "skip" "--startblock" "start_block" "--endblock" "end_block" "--heapallindexed" "--parent-check" "--rootdescend" "--host" "localhost" "--port" "5432" "--username" "username" "--no-password" "--password" "--maintenance-db" "maintenance_db" "--echo" "--jobs" "jobs" "--progress" "--verbose" "--version" "--install-missing" "--help""#
            ),
            command.to_command_string()
        );
    }
}
