use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::fmt::Display;
use std::path::PathBuf;

/// `pg_ctl` is a utility to initialize, start, stop, or control a `PostgreSQL` server.
#[derive(Clone, Debug, Default)]
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::struct_excessive_bools)]
pub struct PgCtlBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    mode: Option<Mode>,
    pgdata: Option<PathBuf>,
    silent: bool,
    timeout: Option<u16>,
    version: bool,
    wait: bool,
    no_wait: bool,
    help: bool,
    core_files: bool,
    log: Option<PathBuf>,
    options: Vec<OsString>,
    path_to_postgres: Option<OsString>,
    shutdown_mode: Option<ShutdownMode>,
    signal: Option<OsString>,
    pid: Option<OsString>,
}

#[derive(Clone, Debug)]
pub enum Mode {
    InitDb,
    Kill,
    LogRotate,
    Promote,
    Restart,
    Reload,
    Start,
    Stop,
    Status,
}

impl Display for Mode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::InitDb => write!(formatter, "initdb"),
            Mode::Kill => write!(formatter, "kill"),
            Mode::LogRotate => write!(formatter, "logrotate"),
            Mode::Promote => write!(formatter, "promote"),
            Mode::Restart => write!(formatter, "restart"),
            Mode::Reload => write!(formatter, "reload"),
            Mode::Start => write!(formatter, "start"),
            Mode::Stop => write!(formatter, "stop"),
            Mode::Status => write!(formatter, "status"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ShutdownMode {
    Smart,
    Fast,
    Immediate,
}

impl Display for ShutdownMode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShutdownMode::Smart => write!(formatter, "smart"),
            ShutdownMode::Fast => write!(formatter, "fast"),
            ShutdownMode::Immediate => write!(formatter, "immediate"),
        }
    }
}

impl PgCtlBuilder {
    /// Create a new [`PgCtlBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgCtlBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new().program_dir(settings.get_binary_dir())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// mode
    #[must_use]
    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// location of the database storage area
    #[must_use]
    pub fn pgdata<P: Into<PathBuf>>(mut self, pgdata: P) -> Self {
        self.pgdata = Some(pgdata.into());
        self
    }

    /// only print errors, no informational messages
    #[must_use]
    pub fn silent(mut self) -> Self {
        self.silent = true;
        self
    }

    /// seconds to wait when using -w option
    #[must_use]
    pub fn timeout(mut self, timeout: u16) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// output version information, then exit
    #[must_use]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// wait until operation completes (default)
    #[must_use]
    pub fn wait(mut self) -> Self {
        self.wait = true;
        self
    }

    /// do not wait until operation completes
    #[must_use]
    pub fn no_wait(mut self) -> Self {
        self.no_wait = true;
        self
    }

    /// show help, then exit
    #[must_use]
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }

    /// allow postgres to produce core files
    #[must_use]
    pub fn core_files(mut self) -> Self {
        self.core_files = true;
        self
    }

    /// write (or append) server log to FILENAME
    #[must_use]
    pub fn log<P: Into<PathBuf>>(mut self, log: P) -> Self {
        self.log = Some(log.into());
        self
    }

    /// command line options to pass to postgres (`PostgreSQL` server executable) or initdb
    #[must_use]
    pub fn options<S: AsRef<OsStr>>(mut self, options: &[S]) -> Self {
        self.options = options.iter().map(|s| s.as_ref().to_os_string()).collect();
        self
    }

    /// normally not necessary
    #[must_use]
    pub fn path_to_postgres<S: AsRef<OsStr>>(mut self, path_to_postgres: S) -> Self {
        self.path_to_postgres = Some(path_to_postgres.as_ref().to_os_string());
        self
    }

    /// MODE can be "smart", "fast", or "immediate"
    #[must_use]
    pub fn shutdown_mode(mut self, shutdown_mode: ShutdownMode) -> Self {
        self.shutdown_mode = Some(shutdown_mode);
        self
    }

    /// SIGNALNAME
    #[must_use]
    pub fn signal<S: AsRef<OsStr>>(mut self, signal: S) -> Self {
        self.signal = Some(signal.as_ref().to_os_string());
        self
    }

    /// PID
    #[must_use]
    pub fn pid<S: AsRef<OsStr>>(mut self, pid: S) -> Self {
        self.pid = Some(pid.as_ref().to_os_string());
        self
    }
}

impl CommandBuilder for PgCtlBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_ctl".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(mode) = &self.mode {
            args.push(mode.to_string().into());
        }

        if let Some(pgdata) = &self.pgdata {
            args.push("--pgdata".into());
            args.push(pgdata.into());
        }

        if self.silent {
            args.push("--silent".into());
        }

        if let Some(timeout) = &self.timeout {
            args.push("--timeout".into());
            args.push(timeout.to_string().into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.wait {
            args.push("--wait".into());
        }

        if self.no_wait {
            args.push("--no-wait".into());
        }

        if self.help {
            args.push("--help".into());
        }

        if self.core_files {
            args.push("--core-files".into());
        }

        if let Some(log) = &self.log {
            args.push("--log".into());
            args.push(log.into());
        }

        for option in &self.options {
            args.push("-o".into());
            args.push(option.into());
        }

        if let Some(path_to_postgres) = &self.path_to_postgres {
            args.push("-p".into());
            args.push(path_to_postgres.into());
        }

        if let Some(shutdown_mode) = &self.shutdown_mode {
            args.push("--mode".into());
            args.push(shutdown_mode.to_string().into());
        }

        if let Some(signal) = &self.signal {
            args.push(signal.into());
        }

        if let Some(pid) = &self.pid {
            args.push(pid.into());
        }

        args
    }

    /// Get the environment variables for the command
    fn get_envs(&self) -> Vec<(OsString, OsString)> {
        self.envs.clone()
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
    fn test_display_mode() {
        assert_eq!("initdb", Mode::InitDb.to_string());
        assert_eq!("kill", Mode::Kill.to_string());
        assert_eq!("logrotate", Mode::LogRotate.to_string());
        assert_eq!("promote", Mode::Promote.to_string());
        assert_eq!("restart", Mode::Restart.to_string());
        assert_eq!("reload", Mode::Reload.to_string());
        assert_eq!("start", Mode::Start.to_string());
        assert_eq!("stop", Mode::Stop.to_string());
        assert_eq!("status", Mode::Status.to_string());
    }

    #[test]
    fn test_display_shutdown_mode() {
        assert_eq!("smart", ShutdownMode::Smart.to_string());
        assert_eq!("fast", ShutdownMode::Fast.to_string());
        assert_eq!("immediate", ShutdownMode::Immediate.to_string());
    }

    #[test]
    fn test_builder_new() {
        let command = PgCtlBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_ctl"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgCtlBuilder::from(&TestSettings).build();
        assert_eq!(r#""./pg_ctl""#, command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = PgCtlBuilder::new()
            .env("PGDATABASE", "database")
            .mode(Mode::Start)
            .pgdata("pgdata")
            .silent()
            .timeout(60)
            .version()
            .wait()
            .no_wait()
            .help()
            .core_files()
            .log("log")
            .options(&["-c log_connections=on"])
            .path_to_postgres("path_to_postgres")
            .shutdown_mode(ShutdownMode::Smart)
            .signal("HUP")
            .pid("12345")
            .build();

        assert_eq!(
            r#"PGDATABASE="database" "pg_ctl" "start" "--pgdata" "pgdata" "--silent" "--timeout" "60" "--version" "--wait" "--no-wait" "--help" "--core-files" "--log" "log" "-o" "-c log_connections=on" "-p" "path_to_postgres" "--mode" "smart" "HUP" "12345""#,
            command.to_command_string()
        );
    }
}
