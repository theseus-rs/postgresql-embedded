use crate::traits::CommandBuilder;
use crate::Settings;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `dropdb` removes a `PostgreSQL` database.
#[derive(Clone, Debug, Default)]
pub struct DropDbBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    echo: bool,
    force: bool,
    interactive: bool,
    version: bool,
    if_exists: bool,
    help: bool,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
    pg_password: Option<OsString>,
    maintenance_db: Option<OsString>,
    dbname: Option<OsString>,
}

impl DropDbBuilder {
    /// Create a new [`DropDbBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`DropDbBuilder`] from [Settings]
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

    /// Show the commands being sent to the server
    #[must_use]
    pub fn echo(mut self) -> Self {
        self.echo = true;
        self
    }

    /// Try to terminate other connections before dropping
    #[must_use]
    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }

    /// Prompt before deleting anything
    #[must_use]
    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }

    /// Output version information, then exit
    #[must_use]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// Don't report error if database doesn't exist
    #[must_use]
    pub fn if_exists(mut self) -> Self {
        self.if_exists = true;
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
}

impl CommandBuilder for DropDbBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "dropdb".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if self.echo {
            args.push("--echo".into());
        }

        if self.force {
            args.push("--force".into());
        }

        if self.interactive {
            args.push("--interactive".into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.if_exists {
            args.push("--if-exists".into());
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

        if let Some(db) = &self.maintenance_db {
            args.push("--maintenance-db".into());
            args.push(db.into());
        }

        if let Some(dbname) = &self.dbname {
            args.push(dbname.into());
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
        let command = DropDbBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("dropdb"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = DropDbBuilder::from(&TestSettings).build();
        assert_eq!(
            r#"PGPASSWORD="password" "./dropdb" "--host" "localhost" "--port" "5432" "--username" "postgres""#,
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder() {
        let command = DropDbBuilder::new()
            .env("PGDATABASE", "database")
            .echo()
            .force()
            .interactive()
            .version()
            .if_exists()
            .help()
            .host("localhost")
            .port(5432)
            .username("postgres")
            .no_password()
            .password()
            .pg_password("password")
            .maintenance_db("postgres")
            .dbname("dbname")
            .build();

        assert_eq!(
            r#"PGDATABASE="database" PGPASSWORD="password" "dropdb" "--echo" "--force" "--interactive" "--version" "--if-exists" "--help" "--host" "localhost" "--port" "5432" "--username" "postgres" "--no-password" "--password" "--maintenance-db" "postgres" "dbname""#,
            command.to_command_string()
        );
    }
}
