use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_dump` dumps a database as a text file or to other formats.
#[derive(Clone, Debug, Default)]
pub struct PgDumpBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    data_only: bool,
    large_objects: bool,
    no_large_objects: bool,
    clean: bool,
    create: bool,
    extension: Option<OsString>,
    encoding: Option<OsString>,
    file: Option<OsString>,
    format: Option<OsString>,
    jobs: Option<OsString>,
    schema: Option<OsString>,
    exclude_schema: Option<OsString>,
    no_owner: bool,
    no_reconnect: bool,
    schema_only: bool,
    superuser: Option<OsString>,
    table: Option<OsString>,
    exclude_table: Option<OsString>,
    verbose: bool,
    version: bool,
    no_privileges: bool,
    compression: Option<OsString>,
    binary_upgrade: bool,
    column_inserts: bool,
    attribute_inserts: bool,
    disable_dollar_quoting: bool,
    disable_triggers: bool,
    enable_row_security: bool,
    exclude_table_data_and_children: Option<OsString>,
    extra_float_digits: Option<OsString>,
    if_exists: bool,
    include_foreign_data: Option<OsString>,
    inserts: bool,
    load_via_partition_root: bool,
    lock_wait_timeout: Option<u16>,
    no_comments: bool,
    no_publications: bool,
    no_security_labels: bool,
    no_subscriptions: bool,
    no_table_access_method: bool,
    no_tablespaces: bool,
    no_toast_compression: bool,
    no_unlogged_table_data: bool,
    on_conflict_do_nothing: bool,
    quote_all_identifiers: bool,
    rows_per_insert: Option<u64>,
    section: Option<OsString>,
    serializable_deferrable: bool,
    snapshot: Option<OsString>,
    strict_names: bool,
    table_and_children: Option<OsString>,
    use_set_session_authorization: bool,
    help: bool,
    dbname: Option<OsString>,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
    pg_password: Option<OsString>,
    role: Option<OsString>,
}

impl PgDumpBuilder {
    /// Create a new [`PgDumpBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgDumpBuilder`] from [Settings]
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

    /// Dump only the data, not the schema
    #[must_use]
    pub fn data_only(mut self) -> Self {
        self.data_only = true;
        self
    }

    /// Dump large objects in binary format
    #[must_use]
    pub fn large_objects(mut self) -> Self {
        self.large_objects = true;
        self
    }

    /// Do not dump large objects
    #[must_use]
    pub fn no_large_objects(mut self) -> Self {
        self.no_large_objects = true;
        self
    }

    /// Output commands to clean (drop) database objects prior to outputting the commands for creating them
    #[must_use]
    pub fn clean(mut self) -> Self {
        self.clean = true;
        self
    }

    /// Output commands to create the database objects (data definition)
    #[must_use]
    pub fn create(mut self) -> Self {
        self.create = true;
        self
    }

    /// Dump data for the named extension
    #[must_use]
    pub fn extension<S: AsRef<OsStr>>(mut self, extension: S) -> Self {
        self.extension = Some(extension.as_ref().to_os_string());
        self
    }

    /// Dump data in encoding ENCODING
    #[must_use]
    pub fn encoding<S: AsRef<OsStr>>(mut self, encoding: S) -> Self {
        self.encoding = Some(encoding.as_ref().to_os_string());
        self
    }

    /// Set the output file or directory name
    #[must_use]
    pub fn file<S: AsRef<OsStr>>(mut self, file: S) -> Self {
        self.file = Some(file.as_ref().to_os_string());
        self
    }

    /// Set the output file format (custom, directory, tar, plain text (default))
    #[must_use]
    pub fn format<S: AsRef<OsStr>>(mut self, format: S) -> Self {
        self.format = Some(format.as_ref().to_os_string());
        self
    }

    /// Use this many parallel jobs to dump
    #[must_use]
    pub fn jobs<S: AsRef<OsStr>>(mut self, jobs: S) -> Self {
        self.jobs = Some(jobs.as_ref().to_os_string());
        self
    }

    /// Dump data for the named schema(s) only
    #[must_use]
    pub fn schema<S: AsRef<OsStr>>(mut self, schema: S) -> Self {
        self.schema = Some(schema.as_ref().to_os_string());
        self
    }

    /// Do not output commands to set ownership of objects to match the original database
    #[must_use]
    pub fn exclude_schema<S: AsRef<OsStr>>(mut self, exclude_schema: S) -> Self {
        self.exclude_schema = Some(exclude_schema.as_ref().to_os_string());
        self
    }

    /// Do not output commands to set ownership of objects to match the original database
    #[must_use]
    pub fn no_owner(mut self) -> Self {
        self.no_owner = true;
        self
    }

    /// Do not reconnect to the database
    #[must_use]
    pub fn no_reconnect(mut self) -> Self {
        self.no_reconnect = true;
        self
    }

    /// Dump only the schema, no data
    #[must_use]
    pub fn schema_only(mut self) -> Self {
        self.schema_only = true;
        self
    }

    /// Dump data as a superuser
    #[must_use]
    pub fn superuser<S: AsRef<OsStr>>(mut self, superuser: S) -> Self {
        self.superuser = Some(superuser.as_ref().to_os_string());
        self
    }

    /// Dump data for the named table(s) only
    #[must_use]
    pub fn table<S: AsRef<OsStr>>(mut self, table: S) -> Self {
        self.table = Some(table.as_ref().to_os_string());
        self
    }

    /// Do not output commands to create the table(s) containing the data
    #[must_use]
    pub fn exclude_table<S: AsRef<OsStr>>(mut self, exclude_table: S) -> Self {
        self.exclude_table = Some(exclude_table.as_ref().to_os_string());
        self
    }

    /// Enable verbose mode
    #[must_use]
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// Output version information, then exit
    #[must_use]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// Do not output commands to set object privileges
    #[must_use]
    pub fn no_privileges(mut self) -> Self {
        self.no_privileges = true;
        self
    }

    /// Set the compression level to use
    #[must_use]
    pub fn compression<S: AsRef<OsStr>>(mut self, compress: S) -> Self {
        self.compression = Some(compress.as_ref().to_os_string());
        self
    }

    /// Dump data in a format suitable for binary upgrade
    #[must_use]
    pub fn binary_upgrade(mut self) -> Self {
        self.binary_upgrade = true;
        self
    }

    /// Dump data as INSERT commands with column names
    #[must_use]
    pub fn column_inserts(mut self) -> Self {
        self.column_inserts = true;
        self
    }

    /// Dump data as INSERT commands with column names
    #[must_use]
    pub fn attribute_inserts(mut self) -> Self {
        self.attribute_inserts = true;
        self
    }

    /// Disable dollar quoting, use SQL standard quoting
    #[must_use]
    pub fn disable_dollar_quoting(mut self) -> Self {
        self.disable_dollar_quoting = true;
        self
    }

    /// Disable triggers during data-only restore
    #[must_use]
    pub fn disable_triggers(mut self) -> Self {
        self.disable_triggers = true;
        self
    }

    /// Dump data with row security enabled
    #[must_use]
    pub fn enable_row_security(mut self) -> Self {
        self.enable_row_security = true;
        self
    }

    /// Dump data for the named table(s) but exclude data for their child tables
    #[must_use]
    pub fn exclude_table_data_and_children<S: AsRef<OsStr>>(
        mut self,
        exclude_table_data_and_children: S,
    ) -> Self {
        self.exclude_table_data_and_children =
            Some(exclude_table_data_and_children.as_ref().to_os_string());
        self
    }

    /// Set the number of digits displayed for floating-point values
    #[must_use]
    pub fn extra_float_digits<S: AsRef<OsStr>>(mut self, extra_float_digits: S) -> Self {
        self.extra_float_digits = Some(extra_float_digits.as_ref().to_os_string());
        self
    }

    /// Use IF EXISTS when dropping objects
    #[must_use]
    pub fn if_exists(mut self) -> Self {
        self.if_exists = true;
        self
    }

    /// Include foreign-data wrappers in the dump
    #[must_use]
    pub fn include_foreign_data<S: AsRef<OsStr>>(mut self, include_foreign_data: S) -> Self {
        self.include_foreign_data = Some(include_foreign_data.as_ref().to_os_string());
        self
    }

    /// Dump data as INSERT commands
    #[must_use]
    pub fn inserts(mut self) -> Self {
        self.inserts = true;
        self
    }

    /// Load data via the partition root table
    #[must_use]
    pub fn load_via_partition_root(mut self) -> Self {
        self.load_via_partition_root = true;
        self
    }

    /// Fail after waiting TIMEOUT for a table lock
    #[must_use]
    pub fn lock_wait_timeout(mut self, lock_wait_timeout: u16) -> Self {
        self.lock_wait_timeout = Some(lock_wait_timeout);
        self
    }

    /// Do not output comments
    #[must_use]
    pub fn no_comments(mut self) -> Self {
        self.no_comments = true;
        self
    }

    /// Do not output publications
    #[must_use]
    pub fn no_publications(mut self) -> Self {
        self.no_publications = true;
        self
    }

    /// Do not output security labels
    #[must_use]
    pub fn no_security_labels(mut self) -> Self {
        self.no_security_labels = true;
        self
    }

    /// Do not output subscriptions
    #[must_use]
    pub fn no_subscriptions(mut self) -> Self {
        self.no_subscriptions = true;
        self
    }

    /// Do not output table access method
    #[must_use]
    pub fn no_table_access_method(mut self) -> Self {
        self.no_table_access_method = true;
        self
    }

    /// Do not output tablespace assignments
    #[must_use]
    pub fn no_tablespaces(mut self) -> Self {
        self.no_tablespaces = true;
        self
    }

    /// Do not output TOAST table compression
    #[must_use]
    pub fn no_toast_compression(mut self) -> Self {
        self.no_toast_compression = true;
        self
    }

    /// Do not output unlogged table data
    #[must_use]
    pub fn no_unlogged_table_data(mut self) -> Self {
        self.no_unlogged_table_data = true;
        self
    }

    /// Use ON CONFLICT DO NOTHING for INSERTs
    #[must_use]
    pub fn on_conflict_do_nothing(mut self) -> Self {
        self.on_conflict_do_nothing = true;
        self
    }

    /// Quote all identifiers, even if not key words
    #[must_use]
    pub fn quote_all_identifiers(mut self) -> Self {
        self.quote_all_identifiers = true;
        self
    }

    /// Set the number of rows per INSERT
    #[must_use]
    pub fn rows_per_insert(mut self, rows_per_insert: u64) -> Self {
        self.rows_per_insert = Some(rows_per_insert);
        self
    }

    /// Dump data for the named section(s) only
    #[must_use]
    pub fn section<S: AsRef<OsStr>>(mut self, section: S) -> Self {
        self.section = Some(section.as_ref().to_os_string());
        self
    }

    /// Dump data as a serializable transaction
    #[must_use]
    pub fn serializable_deferrable(mut self) -> Self {
        self.serializable_deferrable = true;
        self
    }

    /// Use a snapshot with the specified name
    #[must_use]
    pub fn snapshot<S: AsRef<OsStr>>(mut self, snapshot: S) -> Self {
        self.snapshot = Some(snapshot.as_ref().to_os_string());
        self
    }

    /// Use strict SQL identifier syntax
    #[must_use]
    pub fn strict_names(mut self) -> Self {
        self.strict_names = true;
        self
    }

    /// Dump data for the named table(s) and their children
    #[must_use]
    pub fn table_and_children<S: AsRef<OsStr>>(mut self, table_and_children: S) -> Self {
        self.table_and_children = Some(table_and_children.as_ref().to_os_string());
        self
    }

    /// Use SET SESSION AUTHORIZATION commands instead of ALTER OWNER
    #[must_use]
    pub fn use_set_session_authorization(mut self) -> Self {
        self.use_set_session_authorization = true;
        self
    }

    /// Show help, then exit
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

    /// database server port
    #[must_use]
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// database user name
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

    /// Specifies a role name to be used to create the dump
    #[must_use]
    pub fn role<S: AsRef<OsStr>>(mut self, rolename: S) -> Self {
        self.role = Some(rolename.as_ref().to_os_string());
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
    #[expect(clippy::too_many_lines)]
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if self.data_only {
            args.push("--data-only".into());
        }

        if self.large_objects {
            args.push("--large-objects".into());
        }

        if self.no_large_objects {
            args.push("--no-large-objects".into());
        }

        if self.clean {
            args.push("--clean".into());
        }

        if self.create {
            args.push("--create".into());
        }

        if let Some(extension) = &self.extension {
            args.push("--extension".into());
            args.push(extension.into());
        }

        if let Some(encoding) = &self.encoding {
            args.push("--encoding".into());
            args.push(encoding.into());
        }

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

        if let Some(schema) = &self.schema {
            args.push("--schema".into());
            args.push(schema.into());
        }

        if let Some(exclude_schema) = &self.exclude_schema {
            args.push("--exclude-schema".into());
            args.push(exclude_schema.into());
        }

        if self.no_owner {
            args.push("--no-owner".into());
        }

        if self.no_reconnect {
            args.push("--no-reconnect".into());
        }

        if self.schema_only {
            args.push("--schema-only".into());
        }

        if let Some(superuser) = &self.superuser {
            args.push("--superuser".into());
            args.push(superuser.into());
        }

        if let Some(table) = &self.table {
            args.push("--table".into());
            args.push(table.into());
        }

        if let Some(exclude_table) = &self.exclude_table {
            args.push("--exclude-table".into());
            args.push(exclude_table.into());
        }

        if self.verbose {
            args.push("--verbose".into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.no_privileges {
            args.push("--no-privileges".into());
        }

        if let Some(compression) = &self.compression {
            args.push("--compression".into());
            args.push(compression.into());
        }

        if self.binary_upgrade {
            args.push("--binary-upgrade".into());
        }

        if self.column_inserts {
            args.push("--column-inserts".into());
        }

        if self.attribute_inserts {
            args.push("--attribute-inserts".into());
        }

        if self.disable_dollar_quoting {
            args.push("--disable-dollar-quoting".into());
        }

        if self.disable_triggers {
            args.push("--disable-triggers".into());
        }

        if self.enable_row_security {
            args.push("--enable-row-security".into());
        }

        if let Some(exclude_table_data_and_children) = &self.exclude_table_data_and_children {
            args.push("--exclude-table-data-and-children".into());
            args.push(exclude_table_data_and_children.into());
        }

        if let Some(extra_float_digits) = &self.extra_float_digits {
            args.push("--extra-float-digits".into());
            args.push(extra_float_digits.into());
        }

        if self.if_exists {
            args.push("--if-exists".into());
        }

        if let Some(include_foreign_data) = &self.include_foreign_data {
            args.push("--include-foreign-data".into());
            args.push(include_foreign_data.into());
        }

        if self.inserts {
            args.push("--inserts".into());
        }

        if self.load_via_partition_root {
            args.push("--load-via-partition-root".into());
        }

        if let Some(lock_wait_timeout) = &self.lock_wait_timeout {
            args.push("--lock-wait-timeout".into());
            args.push(lock_wait_timeout.to_string().into());
        }

        if self.no_comments {
            args.push("--no-comments".into());
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

        if self.no_toast_compression {
            args.push("--no-toast-compression".into());
        }

        if self.no_unlogged_table_data {
            args.push("--no-unlogged-table-data".into());
        }

        if self.on_conflict_do_nothing {
            args.push("--on-conflict-do-nothing".into());
        }

        if self.quote_all_identifiers {
            args.push("--quote-all-identifiers".into());
        }

        if let Some(rows_per_insert) = &self.rows_per_insert {
            args.push("--rows-per-insert".into());
            args.push(rows_per_insert.to_string().into());
        }

        if let Some(section) = &self.section {
            args.push("--section".into());
            args.push(section.into());
        }

        if self.serializable_deferrable {
            args.push("--serializable-deferrable".into());
        }

        if let Some(snapshot) = &self.snapshot {
            args.push("--snapshot".into());
            args.push(snapshot.into());
        }

        if self.strict_names {
            args.push("--strict-names".into());
        }

        if let Some(table_and_children) = &self.table_and_children {
            args.push("--table-and-children".into());
            args.push(table_and_children.into());
        }

        if self.use_set_session_authorization {
            args.push("--use-set-session-authorization".into());
        }

        if self.help {
            args.push("--help".into());
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
        let command = PgDumpBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_dump"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgDumpBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGPASSWORD="password" "./pg_dump" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\pg_dump" "#;

        assert_eq!(
            format!(
                r#"{command_prefix}"--host" "localhost" "--port" "5432" "--username" "postgres""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder() {
        let command = PgDumpBuilder::new()
            .env("PGDATABASE", "database")
            .data_only()
            .large_objects()
            .no_large_objects()
            .clean()
            .create()
            .extension("extension")
            .encoding("UTF8")
            .file("file")
            .format("format")
            .jobs("jobs")
            .schema("schema")
            .exclude_schema("exclude_schema")
            .no_owner()
            .no_reconnect()
            .schema_only()
            .superuser("superuser")
            .table("table")
            .exclude_table("exclude_table")
            .verbose()
            .version()
            .no_privileges()
            .compression("compression")
            .binary_upgrade()
            .column_inserts()
            .attribute_inserts()
            .disable_dollar_quoting()
            .disable_triggers()
            .enable_row_security()
            .exclude_table_data_and_children("exclude_table_data_and_children")
            .extra_float_digits("extra_float_digits")
            .if_exists()
            .include_foreign_data("include_foreign_data")
            .inserts()
            .load_via_partition_root()
            .lock_wait_timeout(10)
            .no_comments()
            .no_publications()
            .no_security_labels()
            .no_subscriptions()
            .no_table_access_method()
            .no_tablespaces()
            .no_toast_compression()
            .no_unlogged_table_data()
            .on_conflict_do_nothing()
            .quote_all_identifiers()
            .rows_per_insert(100)
            .section("section")
            .serializable_deferrable()
            .snapshot("snapshot")
            .strict_names()
            .table_and_children("table_and_children")
            .use_set_session_authorization()
            .help()
            .dbname("dbname")
            .host("localhost")
            .port(5432)
            .username("postgres")
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
                r#"{command_prefix}"pg_dump" "--data-only" "--large-objects" "--no-large-objects" "--clean" "--create" "--extension" "extension" "--encoding" "UTF8" "--file" "file" "--format" "format" "--jobs" "jobs" "--schema" "schema" "--exclude-schema" "exclude_schema" "--no-owner" "--no-reconnect" "--schema-only" "--superuser" "superuser" "--table" "table" "--exclude-table" "exclude_table" "--verbose" "--version" "--no-privileges" "--compression" "compression" "--binary-upgrade" "--column-inserts" "--attribute-inserts" "--disable-dollar-quoting" "--disable-triggers" "--enable-row-security" "--exclude-table-data-and-children" "exclude_table_data_and_children" "--extra-float-digits" "extra_float_digits" "--if-exists" "--include-foreign-data" "include_foreign_data" "--inserts" "--load-via-partition-root" "--lock-wait-timeout" "10" "--no-comments" "--no-publications" "--no-security-labels" "--no-subscriptions" "--no-table-access-method" "--no-tablespaces" "--no-toast-compression" "--no-unlogged-table-data" "--on-conflict-do-nothing" "--quote-all-identifiers" "--rows-per-insert" "100" "--section" "section" "--serializable-deferrable" "--snapshot" "snapshot" "--strict-names" "--table-and-children" "table_and_children" "--use-set-session-authorization" "--help" "--dbname" "dbname" "--host" "localhost" "--port" "5432" "--username" "postgres" "--no-password" "--password" "--role" "role""#
            ),
            command.to_command_string()
        );
    }
}
