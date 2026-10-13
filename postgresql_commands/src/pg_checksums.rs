use crate::Settings;
use crate::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_checksums` enables, disables, or verifies data checksums in a `PostgreSQL` database cluster.
#[derive(Clone, Debug, Default)]
pub struct PgChecksumsBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    sync_method: Option<OsString>,
    pgdata: Option<PathBuf>,
    check: bool,
    disable: bool,
    enable: bool,
    filenode: Option<OsString>,
    no_sync: bool,
    progress: bool,
    verbose: bool,
    version: bool,
    help: bool,
}

impl PgChecksumsBuilder {
    /// Create a new [`PgChecksumsBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgChecksumsBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new().program_dir(settings.get_binary_dir())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// data directory
    #[must_use]
    pub fn pgdata<P: Into<PathBuf>>(mut self, pgdata: P) -> Self {
        self.pgdata = Some(pgdata.into());
        self
    }

    /// check data checksums (default)
    #[must_use]
    pub fn check(mut self) -> Self {
        self.check = true;
        self
    }

    /// disable data checksums
    #[must_use]
    pub fn disable(mut self) -> Self {
        self.disable = true;
        self
    }

    /// enable data checksums
    #[must_use]
    pub fn enable(mut self) -> Self {
        self.enable = true;
        self
    }

    /// check only relation with specified filenode
    #[must_use]
    pub fn filenode<S: AsRef<OsStr>>(mut self, filenode: S) -> Self {
        self.filenode = Some(filenode.as_ref().to_os_string());
        self
    }

    /// do not wait for changes to be written safely to disk
    #[must_use]
    pub fn no_sync(mut self) -> Self {
        self.no_sync = true;
        self
    }

    /// show progress information
    #[must_use]
    pub fn progress(mut self) -> Self {
        self.progress = true;
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

    /// Select the file synchronization method.
    #[must_use]
    pub fn sync_method<S: AsRef<OsStr>>(mut self, sync_method: S) -> Self {
        self.sync_method = Some(sync_method.as_ref().to_os_string());
        self
    }
}

impl CommandBuilder for PgChecksumsBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_checksums".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if self.version {
            args.push("--version".into());
        }

        if self.help {
            args.push("--help".into());
        }

        if let Some(pgdata) = &self.pgdata {
            args.push("--pgdata".into());
            args.push(pgdata.into());
        }

        if self.check {
            args.push("--check".into());
        }

        if self.disable {
            args.push("--disable".into());
        }

        if self.enable {
            args.push("--enable".into());
        }

        if let Some(filenode) = &self.filenode {
            args.push("--filenode".into());
            args.push(filenode.into());
        }

        if self.no_sync {
            args.push("--no-sync".into());
        }

        if self.progress {
            args.push("--progress".into());
        }

        if self.verbose {
            args.push("--verbose".into());
        }

        if let Some(sync_method) = &self.sync_method {
            args.push("--sync-method".into());
            args.push(sync_method.into());
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
    use crate::CommandExecutor;
    use crate::TestSettings;
    use crate::test_utils::{TestResult, assert_args, binary_dir};
    use crate::traits::CommandToString;
    use test_log::test;

    #[test]
    fn test_builder_new() {
        let command = PgChecksumsBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_checksums"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgChecksumsBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#""./pg_checksums""#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\pg_checksums""#;

        assert_eq!(format!("{command_prefix}"), command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = PgChecksumsBuilder::new()
            .env("PGDATABASE", "database")
            .pgdata("pgdata")
            .check()
            .disable()
            .enable()
            .filenode("12345")
            .no_sync()
            .progress()
            .verbose()
            .version()
            .help()
            .sync_method("fsync")
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"pg_checksums" "--version" "--help" "--pgdata" "pgdata" "--check" "--disable" "--enable" "--filenode" "12345" "--no-sync" "--progress" "--verbose" "--sync-method" "fsync""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_options() {
        assert_args(
            PgChecksumsBuilder::new().sync_method("fsync"),
            &["--sync-method", "fsync"],
        );
    }

    #[test]
    fn test_informational_options_come_first_on_configured_builders() {
        let version = PgChecksumsBuilder::from(&TestSettings).version().get_args();
        assert_eq!(version.first(), Some(&OsString::from("--version")));
        let help = PgChecksumsBuilder::from(&TestSettings).help().get_args();
        assert_eq!(help.first(), Some(&OsString::from("--help")));
    }

    #[test]
    fn test_cli_help_and_version() -> TestResult {
        let Some(bin) = binary_dir() else {
            return Ok(());
        };
        PgChecksumsBuilder::new()
            .program_dir(&bin)
            .version()
            .build()
            .execute()?;
        PgChecksumsBuilder::new()
            .program_dir(&bin)
            .help()
            .build()
            .execute()?;
        Ok(())
    }
}
