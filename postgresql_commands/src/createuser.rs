use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `createuser` creates a new `PostgreSQL` role.
#[derive(Clone, Debug, Default)]
pub struct CreateUserBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    with_admin: Option<OsString>,
    connection_limit: Option<u32>,
    createdb: bool,
    no_createdb: bool,
    echo: bool,
    member_of: Option<OsString>,
    inherit: bool,
    no_inherit: bool,
    login: bool,
    no_login: bool,
    with_member: Option<OsString>,
    pwprompt: bool,
    createrole: bool,
    no_createrole: bool,
    superuser: bool,
    no_superuser: bool,
    valid_until: Option<OsString>,
    version: bool,
    interactive: bool,
    bypassrls: bool,
    no_bypassrls: bool,
    replication: bool,
    no_replication: bool,
    help: bool,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
    pg_password: Option<OsString>,
}

impl CreateUserBuilder {
    /// Create a new [`CreateUserBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`CreateUserBuilder`] from [Settings]
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

    /// ROLE will be a member of new role with admin option
    #[must_use]
    pub fn with_admin<S: AsRef<OsStr>>(mut self, role: S) -> Self {
        self.with_admin = Some(role.as_ref().to_os_string());
        self
    }

    /// Connection limit for role (default: no limit)
    #[must_use]
    pub fn connection_limit(mut self, limit: u32) -> Self {
        self.connection_limit = Some(limit);
        self
    }

    /// Role can create new databases
    #[must_use]
    pub fn createdb(mut self) -> Self {
        self.createdb = true;
        self
    }

    /// Role cannot create databases (default)
    #[must_use]
    pub fn no_createdb(mut self) -> Self {
        self.no_createdb = true;
        self
    }

    /// Show the commands being sent to the server
    #[must_use]
    pub fn echo(mut self) -> Self {
        self.echo = true;
        self
    }

    /// New role will be a member of ROLE
    #[must_use]
    pub fn member_of<S: AsRef<OsStr>>(mut self, role: S) -> Self {
        self.member_of = Some(role.as_ref().to_os_string());
        self
    }

    /// Role inherits privileges of roles it is a member of (default)
    #[must_use]
    pub fn inherit(mut self) -> Self {
        self.inherit = true;
        self
    }

    /// Role does not inherit privileges
    #[must_use]
    pub fn no_inherit(mut self) -> Self {
        self.no_inherit = true;
        self
    }

    /// Role can login (default)
    #[must_use]
    pub fn login(mut self) -> Self {
        self.login = true;
        self
    }

    /// Role cannot login
    #[must_use]
    pub fn no_login(mut self) -> Self {
        self.no_login = true;
        self
    }

    /// ROLE will be a member of new role
    #[must_use]
    pub fn with_member<S: AsRef<OsStr>>(mut self, role: S) -> Self {
        self.with_member = Some(role.as_ref().to_os_string());
        self
    }

    /// Assign a password to new role
    #[must_use]
    pub fn pwprompt(mut self) -> Self {
        self.pwprompt = true;
        self
    }

    /// Role can create new roles
    #[must_use]
    pub fn createrole(mut self) -> Self {
        self.createrole = true;
        self
    }

    /// Role cannot create roles (default)
    #[must_use]
    pub fn no_createrole(mut self) -> Self {
        self.no_createrole = true;
        self
    }

    /// Role will be superuser
    #[must_use]
    pub fn superuser(mut self) -> Self {
        self.superuser = true;
        self
    }

    /// Role will not be superuser (default)
    #[must_use]
    pub fn no_superuser(mut self) -> Self {
        self.no_superuser = true;
        self
    }

    /// Password expiration date and time for role
    #[must_use]
    pub fn valid_until<S: AsRef<OsStr>>(mut self, timestamp: S) -> Self {
        self.valid_until = Some(timestamp.as_ref().to_os_string());
        self
    }

    /// Output version information, then exit
    #[must_use]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// Prompt for missing role name and attributes rather than using defaults
    #[must_use]
    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }

    /// Role can bypass row-level security (RLS) policy
    #[must_use]
    pub fn bypassrls(mut self) -> Self {
        self.bypassrls = true;
        self
    }

    /// Role cannot bypass row-level security (RLS) policy (default)
    #[must_use]
    pub fn no_bypassrls(mut self) -> Self {
        self.no_bypassrls = true;
        self
    }

    /// Role can initiate replication
    #[must_use]
    pub fn replication(mut self) -> Self {
        self.replication = true;
        self
    }

    /// Role cannot initiate replication (default)
    #[must_use]
    pub fn no_replication(mut self) -> Self {
        self.no_replication = true;
        self
    }

    /// Show this help, then exit
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

    /// User name to connect as (not the one to create)
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

impl CommandBuilder for CreateUserBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "createuser".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(role) = &self.with_admin {
            args.push("--with-admin".into());
            args.push(role.into());
        }

        if let Some(limit) = &self.connection_limit {
            args.push("--connection-limit".into());
            args.push(limit.to_string().into());
        }

        if self.createdb {
            args.push("--createdb".into());
        }

        if self.no_createdb {
            args.push("--no-createdb".into());
        }

        if self.echo {
            args.push("--echo".into());
        }

        if let Some(role) = &self.member_of {
            args.push("--member-of".into());
            args.push(role.into());
        }

        if self.inherit {
            args.push("--inherit".into());
        }

        if self.no_inherit {
            args.push("--no-inherit".into());
        }

        if self.login {
            args.push("--login".into());
        }

        if self.no_login {
            args.push("--no-login".into());
        }

        if let Some(role) = &self.with_member {
            args.push("--with-member".into());
            args.push(role.into());
        }

        if self.pwprompt {
            args.push("--pwprompt".into());
        }

        if self.createrole {
            args.push("--createrole".into());
        }

        if self.no_createrole {
            args.push("--no-createrole".into());
        }

        if self.superuser {
            args.push("--superuser".into());
        }

        if self.no_superuser {
            args.push("--no-superuser".into());
        }

        if let Some(timestamp) = &self.valid_until {
            args.push("--valid-until".into());
            args.push(timestamp.into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.interactive {
            args.push("--interactive".into());
        }

        if self.bypassrls {
            args.push("--bypassrls".into());
        }

        if self.no_bypassrls {
            args.push("--no-bypassrls".into());
        }

        if self.replication {
            args.push("--replication".into());
        }

        if self.no_replication {
            args.push("--no-replication".into());
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
        let command = CreateUserBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("createuser"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = CreateUserBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGPASSWORD="password" "./createuser" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\createuser" "#;

        assert_eq!(
            format!(
                r#"{command_prefix}"--host" "localhost" "--port" "5432" "--username" "postgres""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder() {
        let command = CreateUserBuilder::new()
            .env("PGDATABASE", "database")
            .with_admin("admin")
            .connection_limit(10)
            .createdb()
            .no_createdb()
            .echo()
            .member_of("member")
            .inherit()
            .no_inherit()
            .login()
            .no_login()
            .with_member("member")
            .pwprompt()
            .createrole()
            .no_createrole()
            .superuser()
            .no_superuser()
            .valid_until("2021-12-31")
            .version()
            .interactive()
            .bypassrls()
            .no_bypassrls()
            .replication()
            .no_replication()
            .help()
            .host("localhost")
            .port(5432)
            .username("username")
            .no_password()
            .password()
            .pg_password("password")
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGPASSWORD="password" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"createuser" "--with-admin" "admin" "--connection-limit" "10" "--createdb" "--no-createdb" "--echo" "--member-of" "member" "--inherit" "--no-inherit" "--login" "--no-login" "--with-member" "member" "--pwprompt" "--createrole" "--no-createrole" "--superuser" "--no-superuser" "--valid-until" "2021-12-31" "--version" "--interactive" "--bypassrls" "--no-bypassrls" "--replication" "--no-replication" "--help" "--host" "localhost" "--port" "5432" "--username" "username" "--no-password" "--password""#
            ),
            command.to_command_string()
        );
    }
}
