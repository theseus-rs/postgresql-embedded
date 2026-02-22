use crate::Settings;
use crate::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_recvlogical` controls `PostgreSQL` logical decoding streams.
#[derive(Clone, Debug, Default)]
pub struct PgRecvLogicalBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    create_slot: bool,
    drop_slot: bool,
    start: bool,
    endpos: Option<OsString>,
    file: Option<OsString>,
    fsync_interval: Option<OsString>,
    if_not_exists: bool,
    startpos: Option<OsString>,
    no_loop: bool,
    option: Option<OsString>,
    plugin: Option<OsString>,
    status_interval: Option<OsString>,
    slot: Option<OsString>,
    two_phase: bool,
    verbose: bool,
    version: bool,
    help: bool,
    dbname: Option<OsString>,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
    pg_password: Option<OsString>,
}

impl PgRecvLogicalBuilder {
    /// Create a new [`PgRecvLogicalBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgRecvLogicalBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        let mut builder = Self::new()
            .program_dir(settings.get_binary_dir())
            .host(settings.get_host())
            .port(settings.get_port())
            .username(settings.get_username())
            .pg_password(settings.get_password());
        if let Some(socket_dir) = settings.get_socket_dir() {
            builder = builder.host(socket_dir.to_string_lossy().to_string());
        }
        builder
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// create a new replication slot
    #[must_use]
    pub fn create_slot(mut self) -> Self {
        self.create_slot = true;
        self
    }

    /// drop the replication slot
    #[must_use]
    pub fn drop_slot(mut self) -> Self {
        self.drop_slot = true;
        self
    }

    /// start streaming in a replication slot
    #[must_use]
    pub fn start(mut self) -> Self {
        self.start = true;
        self
    }

    /// exit after receiving the specified LSN
    #[must_use]
    pub fn endpos<S: AsRef<OsStr>>(mut self, endpos: S) -> Self {
        self.endpos = Some(endpos.as_ref().to_os_string());
        self
    }

    /// receive log into this file, - for stdout
    #[must_use]
    pub fn file<S: AsRef<OsStr>>(mut self, file: S) -> Self {
        self.file = Some(file.as_ref().to_os_string());
        self
    }

    /// time between fsyncs to the output file (default: 10)
    #[must_use]
    pub fn fsync_interval<S: AsRef<OsStr>>(mut self, fsync_interval: S) -> Self {
        self.fsync_interval = Some(fsync_interval.as_ref().to_os_string());
        self
    }

    /// do not error if slot already exists when creating a slot
    #[must_use]
    pub fn if_not_exists(mut self) -> Self {
        self.if_not_exists = true;
        self
    }

    /// where in an existing slot should the streaming start
    #[must_use]
    pub fn startpos<S: AsRef<OsStr>>(mut self, startpos: S) -> Self {
        self.startpos = Some(startpos.as_ref().to_os_string());
        self
    }

    /// do not loop on connection lost
    #[must_use]
    pub fn no_loop(mut self) -> Self {
        self.no_loop = true;
        self
    }

    /// pass option NAME with optional value VALUE to the output plugin
    #[must_use]
    pub fn option<S: AsRef<OsStr>>(mut self, option: S) -> Self {
        self.option = Some(option.as_ref().to_os_string());
        self
    }

    /// use output plugin PLUGIN (default: `test_decoding`)
    #[must_use]
    pub fn plugin<S: AsRef<OsStr>>(mut self, plugin: S) -> Self {
        self.plugin = Some(plugin.as_ref().to_os_string());
        self
    }

    /// time between status packets sent to server (default: 10)
    #[must_use]
    pub fn status_interval<S: AsRef<OsStr>>(mut self, status_interval: S) -> Self {
        self.status_interval = Some(status_interval.as_ref().to_os_string());
        self
    }

    /// name of the logical replication slot
    #[must_use]
    pub fn slot<S: AsRef<OsStr>>(mut self, slot: S) -> Self {
        self.slot = Some(slot.as_ref().to_os_string());
        self
    }

    /// enable decoding of prepared transactions when creating a slot
    #[must_use]
    pub fn two_phase(mut self) -> Self {
        self.two_phase = true;
        self
    }

    /// output verbose messages
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

    /// database server port number
    #[must_use]
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// connect as specified database user
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
}

impl CommandBuilder for PgRecvLogicalBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_recvlogical".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if self.create_slot {
            args.push("--create-slot".into());
        }

        if self.drop_slot {
            args.push("--drop-slot".into());
        }

        if self.start {
            args.push("--start".into());
        }

        if let Some(endpos) = &self.endpos {
            args.push("--endpos".into());
            args.push(endpos.into());
        }

        if let Some(file) = &self.file {
            args.push("--file".into());
            args.push(file.into());
        }

        if let Some(fsync_interval) = &self.fsync_interval {
            args.push("--fsync-interval".into());
            args.push(fsync_interval.into());
        }

        if self.if_not_exists {
            args.push("--if-not-exists".into());
        }

        if let Some(startpos) = &self.startpos {
            args.push("--startpos".into());
            args.push(startpos.into());
        }

        if self.no_loop {
            args.push("--no-loop".into());
        }

        if let Some(option) = &self.option {
            args.push("--option".into());
            args.push(option.into());
        }

        if let Some(plugin) = &self.plugin {
            args.push("--plugin".into());
            args.push(plugin.into());
        }

        if let Some(status_interval) = &self.status_interval {
            args.push("--status-interval".into());
            args.push(status_interval.into());
        }

        if let Some(slot) = &self.slot {
            args.push("--slot".into());
            args.push(slot.into());
        }

        if self.two_phase {
            args.push("--two-phase".into());
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
    use crate::TestSettings;
    use crate::TestSocketSettings;
    use crate::traits::CommandToString;
    use test_log::test;

    #[test]
    fn test_builder_new() {
        let command = PgRecvLogicalBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_recvlogical"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgRecvLogicalBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGPASSWORD="password" "./pg_recvlogical" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\pg_recvlogical" "#;

        assert_eq!(
            format!(
                r#"{command_prefix}"--host" "localhost" "--port" "5432" "--username" "postgres""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder_from_socket() {
        let command = PgRecvLogicalBuilder::from(&TestSocketSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGPASSWORD="password" "./pg_recvlogical" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\pg_recvlogical" "#;
        assert_eq!(
            format!(
                r#"{command_prefix}"--host" "/tmp/pg_socket" "--port" "5432" "--username" "postgres""#
            ),
            command.to_command_string()
        );
    }
    #[test]
    fn test_builder() {
        let command = PgRecvLogicalBuilder::new()
            .env("PGDATABASE", "database")
            .create_slot()
            .drop_slot()
            .start()
            .endpos("endpos")
            .file("file")
            .fsync_interval("fsync_interval")
            .if_not_exists()
            .startpos("startpos")
            .no_loop()
            .option("option")
            .plugin("plugin")
            .status_interval("status_interval")
            .slot("slot")
            .two_phase()
            .verbose()
            .version()
            .help()
            .dbname("dbname")
            .host("localhost")
            .port(5432)
            .username("username")
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
                r#"{command_prefix}"pg_recvlogical" "--create-slot" "--drop-slot" "--start" "--endpos" "endpos" "--file" "file" "--fsync-interval" "fsync_interval" "--if-not-exists" "--startpos" "startpos" "--no-loop" "--option" "option" "--plugin" "plugin" "--status-interval" "status_interval" "--slot" "slot" "--two-phase" "--verbose" "--version" "--help" "--dbname" "dbname" "--host" "localhost" "--port" "5432" "--username" "username" "--no-password" "--password""#
            ),
            command.to_command_string()
        );
    }
}
