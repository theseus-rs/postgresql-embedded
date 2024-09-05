use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_restore` restores a `PostgreSQL` database from an archive created by `pg_dump`.
#[derive(Clone, Debug, Default)]
pub struct PgRestoreBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    dbname: Option<OsString>,
    file: Option<OsString>,
    format: Option<OsString>,
    list: bool,
    verbose: bool,
    version: bool,
    help: bool,
    data_only: bool,
    clean: bool,
    create: bool,
    exit_on_error: bool,
    index: Option<OsString>,
    jobs: Option<OsString>,
    use_list: Option<OsString>,
    schema: Option<OsString>,
    exclude_schema: Option<OsString>,
    no_owner: bool,
    function: Option<OsString>,
    schema_only: bool,
    superuser: Option<OsString>,
    table: Option<OsString>,
    trigger: Option<OsString>,
    no_privileges: bool,
    single_transaction: bool,
    disable_triggers: bool,
    enable_row_security: bool,
    if_exists: bool,
    no_comments: bool,
    no_data_for_failed_tables: bool,
    no_publications: bool,
    no_security_labels: bool,
    no_subscriptions: bool,
    no_table_access_method: bool,
    no_tablespaces: bool,
    section: Option<OsString>,
    strict_names: bool,
    use_set_session_authorization: bool,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
    pg_password: Option<OsString>,
    role: Option<OsString>,
}

impl PgRestoreBuilder {
    /// Create a new [`PgRestoreBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgRestoreBuilder`] from [Settings]
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

    /// connect to database name
    #[must_use]
    pub fn dbname<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.dbname = Some(name.as_ref().to_os_string());
        self
    }

    /// output file name (- for stdout)
    #[must_use]
    pub fn file<S: AsRef<OsStr>>(mut self, filename: S) -> Self {
        self.file = Some(filename.as_ref().to_os_string());
        self
    }

    /// backup file format (should be automatic)
    #[must_use]
    pub fn format<S: AsRef<OsStr>>(mut self, format: S) -> Self {
        self.format = Some(format.as_ref().to_os_string());
        self
    }

    /// print summarized TOC of the archive
    #[must_use]
    pub fn list(mut self) -> Self {
        self.list = true;
        self
    }

    /// verbose mode
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

    /// show help, then exit
    #[must_use]
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }

    /// restore only the data, no schema
    #[must_use]
    pub fn data_only(mut self) -> Self {
        self.data_only = true;
        self
    }

    /// clean (drop) database objects before recreating
    #[must_use]
    pub fn clean(mut self) -> Self {
        self.clean = true;
        self
    }

    /// create the target database
    #[must_use]
    pub fn create(mut self) -> Self {
        self.create = true;
        self
    }

    /// exit on error, default is to continue
    #[must_use]
    pub fn exit_on_error(mut self) -> Self {
        self.exit_on_error = true;
        self
    }

    /// restore named index
    #[must_use]
    pub fn index<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.index = Some(name.as_ref().to_os_string());
        self
    }

    /// use this many parallel jobs to restore
    #[must_use]
    pub fn jobs<S: AsRef<OsStr>>(mut self, num: S) -> Self {
        self.jobs = Some(num.as_ref().to_os_string());
        self
    }

    /// use table of contents from this file for selecting/ordering output
    #[must_use]
    pub fn use_list<S: AsRef<OsStr>>(mut self, filename: S) -> Self {
        self.use_list = Some(filename.as_ref().to_os_string());
        self
    }

    /// restore only objects in this schema
    #[must_use]
    pub fn schema<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.schema = Some(name.as_ref().to_os_string());
        self
    }

    /// do not restore objects in this schema
    #[must_use]
    pub fn exclude_schema<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.exclude_schema = Some(name.as_ref().to_os_string());
        self
    }

    /// skip restoration of object ownership
    #[must_use]
    pub fn no_owner(mut self) -> Self {
        self.no_owner = true;
        self
    }

    /// restore named function
    #[must_use]
    pub fn function<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.function = Some(name.as_ref().to_os_string());
        self
    }

    /// restore only the schema, no data
    #[must_use]
    pub fn schema_only(mut self) -> Self {
        self.schema_only = true;
        self
    }

    /// superuser user name to use for disabling triggers
    #[must_use]
    pub fn superuser<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.superuser = Some(name.as_ref().to_os_string());
        self
    }

    /// restore named relation (table, view, etc.)
    #[must_use]
    pub fn table<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.table = Some(name.as_ref().to_os_string());
        self
    }

    /// restore named trigger
    #[must_use]
    pub fn trigger<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.trigger = Some(name.as_ref().to_os_string());
        self
    }

    /// skip restoration of access privileges (grant/revoke)
    #[must_use]
    pub fn no_privileges(mut self) -> Self {
        self.no_privileges = true;
        self
    }

    /// restore as a single transaction
    #[must_use]
    pub fn single_transaction(mut self) -> Self {
        self.single_transaction = true;
        self
    }

    /// disable triggers during data-only restore
    #[must_use]
    pub fn disable_triggers(mut self) -> Self {
        self.disable_triggers = true;
        self
    }

    /// enable row security
    #[must_use]
    pub fn enable_row_security(mut self) -> Self {
        self.enable_row_security = true;
        self
    }

    /// use IF EXISTS when dropping objects
    #[must_use]
    pub fn if_exists(mut self) -> Self {
        self.if_exists = true;
        self
    }

    /// do not restore comments
    #[must_use]
    pub fn no_comments(mut self) -> Self {
        self.no_comments = true;
        self
    }

    /// do not restore data of tables that could not be created
    #[must_use]
    pub fn no_data_for_failed_tables(mut self) -> Self {
        self.no_data_for_failed_tables = true;
        self
    }

    /// do not restore publications
    #[must_use]
    pub fn no_publications(mut self) -> Self {
        self.no_publications = true;
        self
    }

    /// do not restore security labels
    #[must_use]
    pub fn no_security_labels(mut self) -> Self {
        self.no_security_labels = true;
        self
    }

    /// do not restore subscriptions
    #[must_use]
    pub fn no_subscriptions(mut self) -> Self {
        self.no_subscriptions = true;
        self
    }

    /// do not restore table access methods
    #[must_use]
    pub fn no_table_access_method(mut self) -> Self {
        self.no_table_access_method = true;
        self
    }

    /// do not restore tablespace assignments
    #[must_use]
    pub fn no_tablespaces(mut self) -> Self {
        self.no_tablespaces = true;
        self
    }

    /// restore named section (pre-data, data, or post-data)
    #[must_use]
    pub fn section<S: AsRef<OsStr>>(mut self, section: S) -> Self {
        self.section = Some(section.as_ref().to_os_string());
        self
    }

    /// require table and/or schema include patterns to match at least one entity each
    #[must_use]
    pub fn strict_names(mut self) -> Self {
        self.strict_names = true;
        self
    }

    /// use SET SESSION AUTHORIZATION commands instead of ALTER OWNER commands to set ownership
    #[must_use]
    pub fn use_set_session_authorization(mut self) -> Self {
        self.use_set_session_authorization = true;
        self
    }

    /// database server host or socket directory
    #[must_use]
    pub fn host<S: AsRef<OsStr>>(mut self, hostname: S) -> Self {
        self.host = Some(hostname.as_ref().to_os_string());
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
    pub fn username<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.username = Some(name.as_ref().to_os_string());
        self
    }

    /// never prompt for password
    #[must_use]
    pub fn no_password(mut self) -> Self {
        self.no_password = true;
        self
    }

    /// force password prompt (should happen automatically)
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

    /// do SET ROLE before restore
    #[must_use]
    pub fn role<S: AsRef<OsStr>>(mut self, rolename: S) -> Self {
        self.role = Some(rolename.as_ref().to_os_string());
        self
    }
}

impl CommandBuilder for PgRestoreBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_restore".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    #[expect(clippy::too_many_lines)]
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(name) = &self.dbname {
            args.push("--dbname".into());
            args.push(name.into());
        }

        if let Some(filename) = &self.file {
            args.push("--file".into());
            args.push(filename.into());
        }

        if let Some(format) = &self.format {
            args.push("--format".into());
            args.push(format.into());
        }

        if self.list {
            args.push("--list".into());
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

        if self.data_only {
            args.push("--data-only".into());
        }

        if self.clean {
            args.push("--clean".into());
        }

        if self.create {
            args.push("--create".into());
        }

        if self.exit_on_error {
            args.push("--exit-on-error".into());
        }

        if let Some(name) = &self.index {
            args.push("--index".into());
            args.push(name.into());
        }

        if let Some(num) = &self.jobs {
            args.push("--jobs".into());
            args.push(num.into());
        }

        if let Some(filename) = &self.use_list {
            args.push("--use-list".into());
            args.push(filename.into());
        }

        if let Some(name) = &self.schema {
            args.push("--schema".into());
            args.push(name.into());
        }

        if let Some(name) = &self.exclude_schema {
            args.push("--exclude-schema".into());
            args.push(name.into());
        }

        if self.no_owner {
            args.push("--no-owner".into());
        }

        if let Some(name) = &self.function {
            args.push("--function".into());
            args.push(name.into());
        }

        if self.schema_only {
            args.push("--schema-only".into());
        }

        if let Some(name) = &self.superuser {
            args.push("--superuser".into());
            args.push(name.into());
        }

        if let Some(name) = &self.table {
            args.push("--table".into());
            args.push(name.into());
        }

        if let Some(name) = &self.trigger {
            args.push("--trigger".into());
            args.push(name.into());
        }

        if self.no_privileges {
            args.push("--no-privileges".into());
        }

        if self.single_transaction {
            args.push("--single-transaction".into());
        }

        if self.disable_triggers {
            args.push("--disable-triggers".into());
        }

        if self.enable_row_security {
            args.push("--enable-row-security".into());
        }

        if self.if_exists {
            args.push("--if-exists".into());
        }

        if self.no_comments {
            args.push("--no-comments".into());
        }

        if self.no_data_for_failed_tables {
            args.push("--no-data-for-failed-tables".into());
        }

        if self.no_publications {
            args.push("--no-publications".into());
        }

        if self.no_security_labels {
            args.push("--no-security-labels".into());
        }

        if self.no_subscriptions {
            args.push("--no-subscriptions".into());
        }

        if self.no_table_access_method {
            args.push("--no-table-access-method".into());
        }

        if self.no_tablespaces {
            args.push("--no-tablespaces".into());
        }

        if let Some(section) = &self.section {
            args.push("--section".into());
            args.push(section.into());
        }

        if self.strict_names {
            args.push("--strict-names".into());
        }

        if self.use_set_session_authorization {
            args.push("--use-set-session-authorization".into());
        }

        if let Some(hostname) = &self.host {
            args.push("--host".into());
            args.push(hostname.into());
        }

        if let Some(port) = &self.port {
            args.push("--port".into());
            args.push(port.to_string().into());
        }

        if let Some(name) = &self.username {
            args.push("--username".into());
            args.push(name.into());
        }

        if self.no_password {
            args.push("--no-password".into());
        }

        if self.password {
            args.push("--password".into());
        }

        if let Some(role) = &self.role {
            args.push("--role".into());
            args.push(role.into());
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
        let command = PgRestoreBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_restore"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgRestoreBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGPASSWORD="password" "./pg_restore" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\pg_restore" "#;

        assert_eq!(
            format!(
                r#"{command_prefix}"--host" "localhost" "--port" "5432" "--username" "postgres""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder() {
        let command = PgRestoreBuilder::new()
            .env("PGDATABASE", "database")
            .dbname("dbname")
            .file("file")
            .format("format")
            .list()
            .verbose()
            .version()
            .help()
            .data_only()
            .clean()
            .create()
            .exit_on_error()
            .index("index")
            .jobs("jobs")
            .use_list("use_list")
            .schema("schema")
            .exclude_schema("exclude_schema")
            .no_owner()
            .function("function")
            .schema_only()
            .superuser("superuser")
            .table("table")
            .trigger("trigger")
            .no_privileges()
            .single_transaction()
            .disable_triggers()
            .enable_row_security()
            .if_exists()
            .no_comments()
            .no_data_for_failed_tables()
            .no_publications()
            .no_security_labels()
            .no_subscriptions()
            .no_table_access_method()
            .no_tablespaces()
            .section("section")
            .strict_names()
            .use_set_session_authorization()
            .host("localhost")
            .port(5432)
            .username("username")
            .no_password()
            .password()
            .pg_password("password")
            .role("role")
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" PGPASSWORD="password" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"pg_restore" "--dbname" "dbname" "--file" "file" "--format" "format" "--list" "--verbose" "--version" "--help" "--data-only" "--clean" "--create" "--exit-on-error" "--index" "index" "--jobs" "jobs" "--use-list" "use_list" "--schema" "schema" "--exclude-schema" "exclude_schema" "--no-owner" "--function" "function" "--schema-only" "--superuser" "superuser" "--table" "table" "--trigger" "trigger" "--no-privileges" "--single-transaction" "--disable-triggers" "--enable-row-security" "--if-exists" "--no-comments" "--no-data-for-failed-tables" "--no-publications" "--no-security-labels" "--no-subscriptions" "--no-table-access-method" "--no-tablespaces" "--section" "section" "--strict-names" "--use-set-session-authorization" "--host" "localhost" "--port" "5432" "--username" "username" "--no-password" "--password" "--role" "role""#
            ),
            command.to_command_string()
        );
    }
}
