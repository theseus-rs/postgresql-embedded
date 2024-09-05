use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `vacuumdb` cleans and analyzes a `PostgreSQL` database.
#[derive(Clone, Debug, Default)]
pub struct VacuumDbBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    all: bool,
    buffer_usage_limit: Option<OsString>,
    dbname: Option<OsString>,
    disable_page_skipping: bool,
    echo: bool,
    full: bool,
    freeze: bool,
    force_index_cleanup: bool,
    jobs: Option<u32>,
    min_mxid_age: Option<OsString>,
    min_xid_age: Option<OsString>,
    no_index_cleanup: bool,
    no_process_main: bool,
    no_process_toast: bool,
    no_truncate: bool,
    schema: Option<OsString>,
    exclude_schema: Option<OsString>,
    parallel: Option<u32>,
    quiet: bool,
    skip_locked: bool,
    table: Option<OsString>,
    verbose: bool,
    version: bool,
    analyze: bool,
    analyze_only: bool,
    analyze_in_stages: bool,
    help: bool,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
    pg_password: Option<OsString>,
    maintenance_db: Option<OsString>,
}

/// vacuumdb cleans and analyzes a `PostgreSQL` database.
impl VacuumDbBuilder {
    /// Create a new [`VacuumDbBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`VacuumDbBuilder`] from [Settings]
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

    /// vacuum all databases
    #[must_use]
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// size of ring buffer used for vacuum
    #[must_use]
    pub fn buffer_usage_limit<S: AsRef<OsStr>>(mut self, buffer_usage_limit: S) -> Self {
        self.buffer_usage_limit = Some(buffer_usage_limit.as_ref().to_os_string());
        self
    }

    /// database to vacuum
    #[must_use]
    pub fn dbname<S: AsRef<OsStr>>(mut self, dbname: S) -> Self {
        self.dbname = Some(dbname.as_ref().to_os_string());
        self
    }

    /// disable all page-skipping behavior
    #[must_use]
    pub fn disable_page_skipping(mut self) -> Self {
        self.disable_page_skipping = true;
        self
    }

    /// show the commands being sent to the server
    #[must_use]
    pub fn echo(mut self) -> Self {
        self.echo = true;
        self
    }

    /// do full vacuuming
    #[must_use]
    pub fn full(mut self) -> Self {
        self.full = true;
        self
    }

    /// freeze row transaction information
    #[must_use]
    pub fn freeze(mut self) -> Self {
        self.freeze = true;
        self
    }

    /// always remove index entries that point to dead tuples
    #[must_use]
    pub fn force_index_cleanup(mut self) -> Self {
        self.force_index_cleanup = true;
        self
    }

    /// use this many concurrent connections to vacuum
    #[must_use]
    pub fn jobs(mut self, jobs: u32) -> Self {
        self.jobs = Some(jobs);
        self
    }

    /// minimum multixact ID age of tables to vacuum
    #[must_use]
    pub fn min_mxid_age<S: AsRef<OsStr>>(mut self, min_mxid_age: S) -> Self {
        self.min_mxid_age = Some(min_mxid_age.as_ref().to_os_string());
        self
    }

    /// minimum transaction ID age of tables to vacuum
    #[must_use]
    pub fn min_xid_age<S: AsRef<OsStr>>(mut self, min_xid_age: S) -> Self {
        self.min_xid_age = Some(min_xid_age.as_ref().to_os_string());
        self
    }

    /// don't remove index entries that point to dead tuples
    #[must_use]
    pub fn no_index_cleanup(mut self) -> Self {
        self.no_index_cleanup = true;
        self
    }

    /// skip the main relation
    #[must_use]
    pub fn no_process_main(mut self) -> Self {
        self.no_process_main = true;
        self
    }

    /// skip the TOAST table associated with the table to vacuum
    #[must_use]
    pub fn no_process_toast(mut self) -> Self {
        self.no_process_toast = true;
        self
    }

    /// don't truncate empty pages at the end of the table
    #[must_use]
    pub fn no_truncate(mut self) -> Self {
        self.no_truncate = true;
        self
    }

    /// vacuum tables in the specified schema(s) only
    #[must_use]
    pub fn schema<S: AsRef<OsStr>>(mut self, schema: S) -> Self {
        self.schema = Some(schema.as_ref().to_os_string());
        self
    }

    /// do not vacuum tables in the specified schema(s)
    #[must_use]
    pub fn exclude_schema<S: AsRef<OsStr>>(mut self, exclude_schema: S) -> Self {
        self.exclude_schema = Some(exclude_schema.as_ref().to_os_string());
        self
    }

    /// use this many background workers for vacuum, if available
    #[must_use]
    pub fn parallel(mut self, parallel: u32) -> Self {
        self.parallel = Some(parallel);
        self
    }

    /// don't write any messages
    #[must_use]
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// skip relations that cannot be immediately locked
    #[must_use]
    pub fn skip_locked(mut self) -> Self {
        self.skip_locked = true;
        self
    }

    /// vacuum specific table(s) only
    #[must_use]
    pub fn table<S: AsRef<OsStr>>(mut self, table: S) -> Self {
        self.table = Some(table.as_ref().to_os_string());
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

    /// update optimizer statistics
    #[must_use]
    pub fn analyze(mut self) -> Self {
        self.analyze = true;
        self
    }

    /// only update optimizer statistics; no vacuum
    #[must_use]
    pub fn analyze_only(mut self) -> Self {
        self.analyze_only = true;
        self
    }

    /// only update optimizer statistics, in multiple stages for faster results; no vacuum
    #[must_use]
    pub fn analyze_in_stages(mut self) -> Self {
        self.analyze_in_stages = true;
        self
    }

    /// show help, then exit
    #[must_use]
    pub fn help(mut self) -> Self {
        self.help = true;
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
}

impl CommandBuilder for VacuumDbBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "vacuumdb".as_ref()
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

        if let Some(buffer_usage_limit) = &self.buffer_usage_limit {
            args.push("--buffer-usage-limit".into());
            args.push(buffer_usage_limit.into());
        }

        if let Some(dbname) = &self.dbname {
            args.push("--dbname".into());
            args.push(dbname.into());
        }

        if self.disable_page_skipping {
            args.push("--disable-page-skipping".into());
        }

        if self.echo {
            args.push("--echo".into());
        }

        if self.full {
            args.push("--full".into());
        }

        if self.freeze {
            args.push("--freeze".into());
        }

        if self.force_index_cleanup {
            args.push("--force-index-cleanup".into());
        }

        if let Some(jobs) = &self.jobs {
            args.push("--jobs".into());
            args.push(jobs.to_string().into());
        }

        if let Some(min_mxid_age) = &self.min_mxid_age {
            args.push("--min-mxid-age".into());
            args.push(min_mxid_age.into());
        }

        if let Some(min_xid_age) = &self.min_xid_age {
            args.push("--min-xid-age".into());
            args.push(min_xid_age.into());
        }

        if self.no_index_cleanup {
            args.push("--no-index-cleanup".into());
        }

        if self.no_process_main {
            args.push("--no-process-main".into());
        }

        if self.no_process_toast {
            args.push("--no-process-toast".into());
        }

        if self.no_truncate {
            args.push("--no-truncate".into());
        }

        if let Some(schema) = &self.schema {
            args.push("--schema".into());
            args.push(schema.into());
        }

        if let Some(exclude_schema) = &self.exclude_schema {
            args.push("--exclude-schema".into());
            args.push(exclude_schema.into());
        }

        if let Some(parallel) = &self.parallel {
            args.push("--parallel".into());
            args.push(parallel.to_string().into());
        }

        if self.quiet {
            args.push("--quiet".into());
        }

        if self.skip_locked {
            args.push("--skip-locked".into());
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

        if self.analyze {
            args.push("--analyze".into());
        }

        if self.analyze_only {
            args.push("--analyze-only".into());
        }

        if self.analyze_in_stages {
            args.push("--analyze-in-stages".into());
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
        let command = VacuumDbBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("vacuumdb"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = VacuumDbBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGPASSWORD="password" "./vacuumdb" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\vacuumdb" "#;

        assert_eq!(
            format!(
                r#"{command_prefix}"--host" "localhost" "--port" "5432" "--username" "postgres""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder() {
        let command = VacuumDbBuilder::new()
            .env("PGDATABASE", "database")
            .all()
            .buffer_usage_limit("buffer_usage_limit")
            .dbname("dbname")
            .disable_page_skipping()
            .echo()
            .full()
            .freeze()
            .force_index_cleanup()
            .jobs(1)
            .min_mxid_age("min_mxid_age")
            .min_xid_age("min_xid_age")
            .no_index_cleanup()
            .no_process_main()
            .no_process_toast()
            .no_truncate()
            .schema("schema")
            .exclude_schema("exclude_schema")
            .parallel(1)
            .quiet()
            .skip_locked()
            .table("table")
            .verbose()
            .version()
            .analyze()
            .analyze_only()
            .analyze_in_stages()
            .help()
            .host("localhost")
            .port(5432)
            .username("username")
            .no_password()
            .password()
            .pg_password("password")
            .maintenance_db("maintenance_db")
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" PGPASSWORD="password" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"vacuumdb" "--all" "--buffer-usage-limit" "buffer_usage_limit" "--dbname" "dbname" "--disable-page-skipping" "--echo" "--full" "--freeze" "--force-index-cleanup" "--jobs" "1" "--min-mxid-age" "min_mxid_age" "--min-xid-age" "min_xid_age" "--no-index-cleanup" "--no-process-main" "--no-process-toast" "--no-truncate" "--schema" "schema" "--exclude-schema" "exclude_schema" "--parallel" "1" "--quiet" "--skip-locked" "--table" "table" "--verbose" "--version" "--analyze" "--analyze-only" "--analyze-in-stages" "--help" "--host" "localhost" "--port" "5432" "--username" "username" "--no-password" "--password" "--maintenance-db" "maintenance_db""#
            ),
            command.to_command_string()
        );
    }
}
