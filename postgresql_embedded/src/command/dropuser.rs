use crate::command::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// dropuser removes a PostgreSQL role.
#[derive(Clone, Debug, Default)]
pub struct DropUserBuilder {
    program_dir: Option<PathBuf>,
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
}

impl DropUserBuilder {
    /// Create a new [`DropUserBuilder`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Location of the program binary
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// Show the commands being sent to the server
    pub fn echo(mut self) -> Self {
        self.echo = true;
        self
    }

    /// Prompt before deleting anything, and prompt for role name if not specified
    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }

    /// Output version information, then exit
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// Don't report error if user doesn't exist
    pub fn if_exists(mut self) -> Self {
        self.if_exists = true;
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

    /// User name to connect as (not the one to drop)
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::traits::CommandToString;

    #[test]
    fn test_builder_new() {
        let command = DropUserBuilder::new().program_dir(".").build();

        assert_eq!(
            PathBuf::from(".").join("dropuser"),
            PathBuf::from(command.to_command_string().replace("\"", ""))
        );
    }

    #[test]
    fn test_builder() {
        let command = DropUserBuilder::new()
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
            .build();

        assert_eq!(
            r#""dropuser" "--echo" "--interactive" "--version" "--if-exists" "--help" "--host" "localhost" "--port" "5432" "--username" "postgres" "--no-password" "--password""#,
            command.to_command_string()
        );
    }
}
