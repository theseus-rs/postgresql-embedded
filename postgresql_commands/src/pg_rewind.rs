use crate::Settings;
use crate::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_rewind` synchronizes a `PostgreSQL` data directory with another data directory.
#[derive(Clone, Debug, Default)]
pub struct PgRewindBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    restore_target_wal: bool,
    target_pgdata: Option<PathBuf>,
    source_pgdata: Option<PathBuf>,
    source_server: Option<OsString>,
    dry_run: bool,
    no_sync: bool,
    progress: bool,
    write_recovery_conf: bool,
    config_file: Option<OsString>,
    debug: bool,
    no_ensure_shutdown: bool,
    version: bool,
    help: bool,
}

impl PgRewindBuilder {
    /// Create a new [`PgRewindBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgRewindBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new().program_dir(settings.get_binary_dir())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// use `restore_command` in target configuration to retrieve WAL files from archives
    #[must_use]
    pub fn restore_target_wal(mut self) -> Self {
        self.restore_target_wal = true;
        self
    }

    /// existing data directory to modify
    #[must_use]
    pub fn target_pgdata<P: Into<PathBuf>>(mut self, directory: P) -> Self {
        self.target_pgdata = Some(directory.into());
        self
    }

    /// source data directory to synchronize with
    #[must_use]
    pub fn source_pgdata<P: Into<PathBuf>>(mut self, directory: P) -> Self {
        self.source_pgdata = Some(directory.into());
        self
    }

    /// source server to synchronize with
    #[must_use]
    pub fn source_server<S: AsRef<OsStr>>(mut self, connstr: S) -> Self {
        self.source_server = Some(connstr.as_ref().to_os_string());
        self
    }

    /// stop before modifying anything
    #[must_use]
    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }

    /// do not wait for changes to be written safely to disk
    #[must_use]
    pub fn no_sync(mut self) -> Self {
        self.no_sync = true;
        self
    }

    /// write progress messages
    #[must_use]
    pub fn progress(mut self) -> Self {
        self.progress = true;
        self
    }

    /// write configuration for replication (requires --source-server)
    #[must_use]
    pub fn write_recovery_conf(mut self) -> Self {
        self.write_recovery_conf = true;
        self
    }

    /// use specified main server configuration file when running target cluster
    #[must_use]
    pub fn config_file<S: AsRef<OsStr>>(mut self, filename: S) -> Self {
        self.config_file = Some(filename.as_ref().to_os_string());
        self
    }

    /// write a lot of debug messages
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    /// do not automatically fix unclean shutdown
    #[must_use]
    pub fn no_ensure_shutdown(mut self) -> Self {
        self.no_ensure_shutdown = true;
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
}

impl CommandBuilder for PgRewindBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_rewind".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if self.restore_target_wal {
            args.push("--restore-target-wal".into());
        }

        if let Some(directory) = &self.target_pgdata {
            args.push("--target-pgdata".into());
            args.push(directory.into());
        }

        if let Some(directory) = &self.source_pgdata {
            args.push("--source-pgdata".into());
            args.push(directory.into());
        }

        if let Some(connstr) = &self.source_server {
            args.push("--source-server".into());
            args.push(connstr.into());
        }

        if self.dry_run {
            args.push("--dry-run".into());
        }

        if self.no_sync {
            args.push("--no-sync".into());
        }

        if self.progress {
            args.push("--progress".into());
        }

        if self.write_recovery_conf {
            args.push("--write-recovery-conf".into());
        }

        if let Some(filename) = &self.config_file {
            args.push("--config-file".into());
            args.push(filename.into());
        }

        if self.debug {
            args.push("--debug".into());
        }

        if self.no_ensure_shutdown {
            args.push("--no-ensure-shutdown".into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.help {
            args.push("--help".into());
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
    use crate::TestSettings;
    use crate::traits::CommandToString;
    use test_log::test;

    #[test]
    fn test_builder_new() {
        let command = PgRewindBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_rewind"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgRewindBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#""./pg_rewind""#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\pg_rewind""#;

        assert_eq!(format!("{command_prefix}"), command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = PgRewindBuilder::new()
            .env("PGDATABASE", "database")
            .restore_target_wal()
            .target_pgdata("target_pgdata")
            .source_pgdata("source_pgdata")
            .source_server("source_server")
            .dry_run()
            .no_sync()
            .progress()
            .write_recovery_conf()
            .config_file("config_file")
            .debug()
            .no_ensure_shutdown()
            .version()
            .help()
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"pg_rewind" "--restore-target-wal" "--target-pgdata" "target_pgdata" "--source-pgdata" "source_pgdata" "--source-server" "source_server" "--dry-run" "--no-sync" "--progress" "--write-recovery-conf" "--config-file" "config_file" "--debug" "--no-ensure-shutdown" "--version" "--help""#
            ),
            command.to_command_string()
        );
    }
}
