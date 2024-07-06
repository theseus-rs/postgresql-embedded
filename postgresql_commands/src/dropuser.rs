use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `dropuser` removes a `PostgreSQL` role.
#[derive(Clone, Debug, Default)]
pub struct DropUserBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    echo: bool,
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
}

impl DropUserBuilder {
    /// Create a new [`DropUserBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`DropUserBuilder`] from [Settings]
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

    /// Prompt before deleting anything, and prompt for role name if not specified
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

    /// Don't report error if user doesn't exist
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

    /// User name to connect as (not the one to drop)
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
}

impl CommandBuilder for DropUserBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "dropuser".as_ref()
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
        let command = DropUserBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("dropuser"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = DropUserBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGPASSWORD="password" "./dropuser" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\dropuser" "#;

        assert_eq!(
            format!(
                r#"{command_prefix}"--host" "localhost" "--port" "5432" "--username" "postgres""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder() {
        let command = DropUserBuilder::new()
            .env("PGDATABASE", "database")
            .echo()
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
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" PGPASSWORD="password" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"dropuser" "--echo" "--interactive" "--version" "--if-exists" "--help" "--host" "localhost" "--port" "5432" "--username" "postgres" "--no-password" "--password""#
            ),
            command.to_command_string()
        );
    }
}
