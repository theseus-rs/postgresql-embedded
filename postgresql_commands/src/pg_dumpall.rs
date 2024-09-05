use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_dumpall` extracts a `PostgreSQL` database cluster into an SQL script file.
#[derive(Clone, Debug, Default)]
pub struct PgDumpAllBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
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
    pg_password: Option<OsString>,
    role: Option<OsString>,
}

impl PgDumpAllBuilder {
    /// Create a new [`PgDumpAllBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgDumpAllBuilder`] from [Settings]
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

    /// output file name
    #[must_use]
    pub fn file<S: AsRef<OsStr>>(mut self, file: S) -> Self {
        self.file = Some(file.as_ref().to_os_string());
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

    /// fail after waiting TIMEOUT for a table lock
    #[must_use]
    pub fn lock_wait_timeout(mut self, lock_wait_timeout: u16) -> Self {
        self.lock_wait_timeout = Some(lock_wait_timeout);
        self
    }

    /// show help, then exit
    #[must_use]
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }

    /// dump only the data, not the schema
    #[must_use]
    pub fn data_only(mut self) -> Self {
        self.data_only = true;
        self
    }

    /// clean (drop) database objects before recreating them
    #[must_use]
    pub fn clean(mut self) -> Self {
        self.clean = true;
        self
    }

    /// encoding for the dump
    #[must_use]
    pub fn encoding<S: AsRef<OsStr>>(mut self, encoding: S) -> Self {
        self.encoding = Some(encoding.as_ref().to_os_string());
        self
    }

    /// dump only global objects, not database-specific objects
    #[must_use]
    pub fn globals_only(mut self) -> Self {
        self.globals_only = true;
        self
    }

    /// do not output commands to set object ownership
    #[must_use]
    pub fn no_owner(mut self) -> Self {
        self.no_owner = true;
        self
    }

    /// dump only the roles, not the role memberships or privileges
    #[must_use]
    pub fn roles_only(mut self) -> Self {
        self.roles_only = true;
        self
    }

    /// dump only the object definitions (schema), not data
    #[must_use]
    pub fn schema_only(mut self) -> Self {
        self.schema_only = true;
        self
    }

    /// superuser user name to use in the dump
    #[must_use]
    pub fn superuser<S: AsRef<OsStr>>(mut self, superuser: S) -> Self {
        self.superuser = Some(superuser.as_ref().to_os_string());
        self
    }

    /// dump only the tablespace definitions
    #[must_use]
    pub fn tablespaces_only(mut self) -> Self {
        self.tablespaces_only = true;
        self
    }

    /// do not dump object privileges (grant/revoke commands)
    #[must_use]
    pub fn no_privileges(mut self) -> Self {
        self.no_privileges = true;
        self
    }

    /// dump in a format suitable for binary upgrade
    #[must_use]
    pub fn binary_upgrade(mut self) -> Self {
        self.binary_upgrade = true;
        self
    }

    /// dump data as INSERT commands with column names
    #[must_use]
    pub fn column_inserts(mut self) -> Self {
        self.column_inserts = true;
        self
    }

    /// disable dollar quoting, use SQL standard quoting
    #[must_use]
    pub fn disable_dollar_quoting(mut self) -> Self {
        self.disable_dollar_quoting = true;
        self
    }

    /// disable triggers during data-only restore
    #[must_use]
    pub fn disable_triggers(mut self) -> Self {
        self.disable_triggers = true;
        self
    }

    /// exclude the named database from the dump
    #[must_use]
    pub fn exclude_database<S: AsRef<OsStr>>(mut self, exclude_database: S) -> Self {
        self.exclude_database = Some(exclude_database.as_ref().to_os_string());
        self
    }

    /// set the number of digits displayed for floating-point values
    #[must_use]
    pub fn extra_float_digits<S: AsRef<OsStr>>(mut self, extra_float_digits: S) -> Self {
        self.extra_float_digits = Some(extra_float_digits.as_ref().to_os_string());
        self
    }

    /// use IF EXISTS when dropping objects
    #[must_use]
    pub fn if_exists(mut self) -> Self {
        self.if_exists = true;
        self
    }

    /// dump data as proper INSERT commands
    #[must_use]
    pub fn inserts(mut self) -> Self {
        self.inserts = true;
        self
    }

    /// load data via the partition root table
    #[must_use]
    pub fn load_via_partition_root(mut self) -> Self {
        self.load_via_partition_root = true;
        self
    }

    /// do not dump comments
    #[must_use]
    pub fn no_comments(mut self) -> Self {
        self.no_comments = true;
        self
    }

    /// do not dump publications
    #[must_use]
    pub fn no_publications(mut self) -> Self {
        self.no_publications = true;
        self
    }

    /// do not dump passwords for roles
    #[must_use]
    pub fn no_role_passwords(mut self) -> Self {
        self.no_role_passwords = true;
        self
    }

    /// do not dump security labels
    #[must_use]
    pub fn no_security_labels(mut self) -> Self {
        self.no_security_labels = true;
        self
    }

    /// do not dump subscriptions
    #[must_use]
    pub fn no_subscriptions(mut self) -> Self {
        self.no_subscriptions = true;
        self
    }

    /// do not wait for changes to be written safely to disk
    #[must_use]
    pub fn no_sync(mut self) -> Self {
        self.no_sync = true;
        self
    }

    /// do not dump table access method information
    #[must_use]
    pub fn no_table_access_method(mut self) -> Self {
        self.no_table_access_method = true;
        self
    }

    /// do not dump tablespace assignments
    #[must_use]
    pub fn no_tablespaces(mut self) -> Self {
        self.no_tablespaces = true;
        self
    }

    /// do not dump TOAST compression information
    #[must_use]
    pub fn no_toast_compression(mut self) -> Self {
        self.no_toast_compression = true;
        self
    }

    /// do not dump unlogged table data
    #[must_use]
    pub fn no_unlogged_table_data(mut self) -> Self {
        self.no_unlogged_table_data = true;
        self
    }

    /// use ON CONFLICT DO NOTHING for INSERTs
    #[must_use]
    pub fn on_conflict_do_nothing(mut self) -> Self {
        self.on_conflict_do_nothing = true;
        self
    }

    /// quote all identifiers, even if not key words
    #[must_use]
    pub fn quote_all_identifiers(mut self) -> Self {
        self.quote_all_identifiers = true;
        self
    }

    /// set the number of rows per INSERT command
    #[must_use]
    pub fn rows_per_insert<S: AsRef<OsStr>>(mut self, rows_per_insert: S) -> Self {
        self.rows_per_insert = Some(rows_per_insert.as_ref().to_os_string());
        self
    }

    /// use SET SESSION AUTHORIZATION commands instead of ALTER OWNER
    #[must_use]
    pub fn use_set_session_authorization(mut self) -> Self {
        self.use_set_session_authorization = true;
        self
    }

    /// database name to connect to
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

    /// database name to connect to
    #[must_use]
    pub fn database<S: AsRef<OsStr>>(mut self, database: S) -> Self {
        self.database = Some(database.as_ref().to_os_string());
        self
    }

    /// database server port number
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

    /// role name to use in the dump
    #[must_use]
    pub fn role<S: AsRef<OsStr>>(mut self, role: S) -> Self {
        self.role = Some(role.as_ref().to_os_string());
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
    #[expect(clippy::too_many_lines)]
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

        if self.data_only {
            args.push("--data-only".into());
        }

        if self.clean {
            args.push("--clean".into());
        }

        if let Some(encoding) = &self.encoding {
            args.push("--encoding".into());
            args.push(encoding.into());
        }

        if self.globals_only {
            args.push("--globals-only".into());
        }

        if self.no_owner {
            args.push("--no-owner".into());
        }

        if self.roles_only {
            args.push("--roles-only".into());
        }

        if self.schema_only {
            args.push("--schema-only".into());
        }

        if let Some(superuser) = &self.superuser {
            args.push("--superuser".into());
            args.push(superuser.into());
        }

        if self.tablespaces_only {
            args.push("--tablespaces-only".into());
        }

        if self.no_privileges {
            args.push("--no-privileges".into());
        }

        if self.binary_upgrade {
            args.push("--binary-upgrade".into());
        }

        if self.column_inserts {
            args.push("--column-inserts".into());
        }

        if self.disable_dollar_quoting {
            args.push("--disable-dollar-quoting".into());
        }

        if self.disable_triggers {
            args.push("--disable-triggers".into());
        }

        if let Some(exclude_database) = &self.exclude_database {
            args.push("--exclude-database".into());
            args.push(exclude_database.into());
        }

        if let Some(extra_float_digits) = &self.extra_float_digits {
            args.push("--extra-float-digits".into());
            args.push(extra_float_digits.into());
        }

        if self.if_exists {
            args.push("--if-exists".into());
        }

        if self.inserts {
            args.push("--inserts".into());
        }

        if self.load_via_partition_root {
            args.push("--load-via-partition-root".into());
        }

        if self.no_comments {
            args.push("--no-comments".into());
        }

        if self.no_publications {
            args.push("--no-publications".into());
        }

        if self.no_role_passwords {
            args.push("--no-role-passwords".into());
        }

        if self.no_security_labels {
            args.push("--no-security-labels".into());
        }

        if self.no_subscriptions {
            args.push("--no-subscriptions".into());
        }

        if self.no_sync {
            args.push("--no-sync".into());
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
            args.push(rows_per_insert.into());
        }

        if self.use_set_session_authorization {
            args.push("--use-set-session-authorization".into());
        }

        if let Some(dbname) = &self.dbname {
            args.push("--dbname".into());
            args.push(dbname.into());
        }

        if let Some(host) = &self.host {
            args.push("--host".into());
            args.push(host.into());
        }

        if let Some(database) = &self.database {
            args.push("--database".into());
            args.push(database.into());
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
        let command = PgDumpAllBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_dumpall"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgDumpAllBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGPASSWORD="password" "./pg_dumpall" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\pg_dumpall" "#;

        assert_eq!(
            format!(
                r#"{command_prefix}"--host" "localhost" "--port" "5432" "--username" "postgres""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder() {
        let command = PgDumpAllBuilder::new()
            .env("PGDATABASE", "database")
            .file("dump.sql")
            .verbose()
            .version()
            .lock_wait_timeout(10)
            .help()
            .data_only()
            .clean()
            .encoding("UTF8")
            .globals_only()
            .no_owner()
            .roles_only()
            .schema_only()
            .superuser("postgres")
            .tablespaces_only()
            .no_privileges()
            .binary_upgrade()
            .column_inserts()
            .disable_dollar_quoting()
            .disable_triggers()
            .exclude_database("exclude")
            .extra_float_digits("2")
            .if_exists()
            .inserts()
            .load_via_partition_root()
            .no_comments()
            .no_publications()
            .no_role_passwords()
            .no_security_labels()
            .no_subscriptions()
            .no_sync()
            .no_table_access_method()
            .no_tablespaces()
            .no_toast_compression()
            .no_unlogged_table_data()
            .on_conflict_do_nothing()
            .quote_all_identifiers()
            .rows_per_insert("1000")
            .use_set_session_authorization()
            .dbname("postgres")
            .host("localhost")
            .database("postgres")
            .port(5432)
            .username("postgres")
            .no_password()
            .password()
            .pg_password("password")
            .role("postgres")
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" PGPASSWORD="password" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"pg_dumpall" "--file" "dump.sql" "--verbose" "--version" "--lock-wait-timeout" "10" "--help" "--data-only" "--clean" "--encoding" "UTF8" "--globals-only" "--no-owner" "--roles-only" "--schema-only" "--superuser" "postgres" "--tablespaces-only" "--no-privileges" "--binary-upgrade" "--column-inserts" "--disable-dollar-quoting" "--disable-triggers" "--exclude-database" "exclude" "--extra-float-digits" "2" "--if-exists" "--inserts" "--load-via-partition-root" "--no-comments" "--no-publications" "--no-role-passwords" "--no-security-labels" "--no-subscriptions" "--no-sync" "--no-table-access-method" "--no-tablespaces" "--no-toast-compression" "--no-unlogged-table-data" "--on-conflict-do-nothing" "--quote-all-identifiers" "--rows-per-insert" "1000" "--use-set-session-authorization" "--dbname" "postgres" "--host" "localhost" "--database" "postgres" "--port" "5432" "--username" "postgres" "--no-password" "--password" "--role" "postgres""#
            ),
            command.to_command_string()
        );
    }
}
