use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `createdb` creates a `PostgreSQL` database.
#[derive(Clone, Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct CreateDbBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    tablespace: Option<OsString>,
    echo: bool,
    encoding: Option<OsString>,
    locale: Option<OsString>,
    lc_collate: Option<OsString>,
    lc_ctype: Option<OsString>,
    icu_locale: Option<OsString>,
    icu_rules: Option<OsString>,
    locale_provider: Option<OsString>,
    owner: Option<OsString>,
    strategy: Option<OsString>,
    template: Option<OsString>,
    version: bool,
    help: bool,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
    pg_password: Option<OsString>,
    maintenance_db: Option<OsString>,
    dbname: Option<OsString>,
    description: Option<OsString>,
}

impl CreateDbBuilder {
    /// Create a new [`CreateDbBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`CreateDbBuilder`] from [Settings]
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

    /// Default tablespace for the database
    #[must_use]
    pub fn tablespace<S: AsRef<OsStr>>(mut self, tablespace: S) -> Self {
        self.tablespace = Some(tablespace.as_ref().to_os_string());
        self
    }

    /// Show the commands being sent to the server
    #[must_use]
    pub fn echo(mut self) -> Self {
        self.echo = true;
        self
    }

    /// Encoding for the database
    #[must_use]
    pub fn encoding<S: AsRef<OsStr>>(mut self, encoding: S) -> Self {
        self.encoding = Some(encoding.as_ref().to_os_string());
        self
    }

    /// Locale settings for the database
    #[must_use]
    pub fn locale<S: AsRef<OsStr>>(mut self, locale: S) -> Self {
        self.locale = Some(locale.as_ref().to_os_string());
        self
    }

    /// `LC_COLLATE` setting for the database
    #[must_use]
    pub fn lc_collate<S: AsRef<OsStr>>(mut self, lc_collate: S) -> Self {
        self.lc_collate = Some(lc_collate.as_ref().to_os_string());
        self
    }

    /// `LC_CTYPE` setting for the database
    #[must_use]
    pub fn lc_ctype<S: AsRef<OsStr>>(mut self, lc_ctype: S) -> Self {
        self.lc_ctype = Some(lc_ctype.as_ref().to_os_string());
        self
    }

    /// ICU locale setting for the database
    #[must_use]
    pub fn icu_locale<S: AsRef<OsStr>>(mut self, icu_locale: S) -> Self {
        self.icu_locale = Some(icu_locale.as_ref().to_os_string());
        self
    }

    /// ICU rules setting for the database
    #[must_use]
    pub fn icu_rules<S: AsRef<OsStr>>(mut self, icu_rules: S) -> Self {
        self.icu_rules = Some(icu_rules.as_ref().to_os_string());
        self
    }

    /// Locale provider for the database's default collation
    #[must_use]
    pub fn locale_provider<S: AsRef<OsStr>>(mut self, locale_provider: S) -> Self {
        self.locale_provider = Some(locale_provider.as_ref().to_os_string());
        self
    }

    /// Database user to own the new database
    #[must_use]
    pub fn owner<S: AsRef<OsStr>>(mut self, owner: S) -> Self {
        self.owner = Some(owner.as_ref().to_os_string());
        self
    }

    /// Database creation strategy `wal_log` or `file_copy`
    #[must_use]
    pub fn strategy<S: AsRef<OsStr>>(mut self, strategy: S) -> Self {
        self.strategy = Some(strategy.as_ref().to_os_string());
        self
    }

    /// Template database to copy
    #[must_use]
    pub fn template<S: AsRef<OsStr>>(mut self, template: S) -> Self {
        self.template = Some(template.as_ref().to_os_string());
        self
    }

    /// Output version information, then exit
    #[must_use]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// Show help, then exit
    #[must_use]
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }

    /// Database server host or socket directory
    #[must_use]
    pub fn host<S: AsRef<OsStr>>(mut self, host: S) -> Self {
        self.host = Some(host.as_ref().to_os_string());
        self
    }

    /// Database server port
    #[must_use]
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// User name to connect as
    #[must_use]
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
        self
    }

    /// Never prompt for password
    #[must_use]
    pub fn no_password(mut self) -> Self {
        self.no_password = true;
        self
    }

    /// Force password prompt
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

    /// Alternate maintenance database
    #[must_use]
    pub fn maintenance_db<S: AsRef<OsStr>>(mut self, db: S) -> Self {
        self.maintenance_db = Some(db.as_ref().to_os_string());
        self
    }

    /// Database name
    #[must_use]
    pub fn dbname<S: AsRef<OsStr>>(mut self, dbname: S) -> Self {
        self.dbname = Some(dbname.as_ref().to_os_string());
        self
    }

    /// Database description
    #[must_use]
    pub fn description<S: AsRef<OsStr>>(mut self, description: S) -> Self {
        self.description = Some(description.as_ref().to_os_string());
        self
    }
}

impl CommandBuilder for CreateDbBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "createdb".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(tablespace) = &self.tablespace {
            args.push("--tablespace".into());
            args.push(tablespace.into());
        }

        if self.echo {
            args.push("--echo".into());
        }

        if let Some(encoding) = &self.encoding {
            args.push("--encoding".into());
            args.push(encoding.into());
        }

        if let Some(locale) = &self.locale {
            args.push("--locale".into());
            args.push(locale.into());
        }

        if let Some(lc_collate) = &self.lc_collate {
            args.push("--lc-collate".into());
            args.push(lc_collate.into());
        }

        if let Some(lc_ctype) = &self.lc_ctype {
            args.push("--lc-ctype".into());
            args.push(lc_ctype.into());
        }

        if let Some(icu_locale) = &self.icu_locale {
            args.push("--icu-locale".into());
            args.push(icu_locale.into());
        }

        if let Some(icu_rules) = &self.icu_rules {
            args.push("--icu-rules".into());
            args.push(icu_rules.into());
        }

        if let Some(locale_provider) = &self.locale_provider {
            args.push("--locale-provider".into());
            args.push(locale_provider.into());
        }

        if let Some(owner) = &self.owner {
            args.push("--owner".into());
            args.push(owner.into());
        }

        if let Some(strategy) = &self.strategy {
            args.push("--strategy".into());
            args.push(strategy.into());
        }

        if let Some(template) = &self.template {
            args.push("--template".into());
            args.push(template.into());
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

        if let Some(dbname) = &self.dbname {
            args.push(dbname.into());
        }

        if let Some(description) = &self.description {
            args.push(description.into());
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
        let command = CreateDbBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("createdb"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = CreateDbBuilder::from(&TestSettings).build();
        assert_eq!(
            r#"PGPASSWORD="password" "./createdb" "--host" "localhost" "--port" "5432" "--username" "postgres""#,
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder() {
        let command = CreateDbBuilder::new()
            .env("PGDATABASE", "database")
            .tablespace("pg_default")
            .echo()
            .encoding("UTF8")
            .locale("en_US.UTF-8")
            .lc_collate("en_US.UTF-8")
            .lc_ctype("en_US.UTF-8")
            .icu_locale("en_US")
            .icu_rules("standard")
            .locale_provider("icu")
            .owner("postgres")
            .strategy("wal_log")
            .template("template0")
            .version()
            .help()
            .host("localhost")
            .port(5432)
            .username("postgres")
            .no_password()
            .password()
            .pg_password("password")
            .maintenance_db("postgres")
            .dbname("testdb")
            .description("Test Database")
            .build();

        assert_eq!(
            r#"PGDATABASE="database" PGPASSWORD="password" "createdb" "--tablespace" "pg_default" "--echo" "--encoding" "UTF8" "--locale" "en_US.UTF-8" "--lc-collate" "en_US.UTF-8" "--lc-ctype" "en_US.UTF-8" "--icu-locale" "en_US" "--icu-rules" "standard" "--locale-provider" "icu" "--owner" "postgres" "--strategy" "wal_log" "--template" "template0" "--version" "--help" "--host" "localhost" "--port" "5432" "--username" "postgres" "--no-password" "--password" "--maintenance-db" "postgres" "testdb" "Test Database""#,
            command.to_command_string()
        );
    }
}
