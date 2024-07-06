use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_archivecleanup` removes older WAL files from `PostgreSQL` archives.
#[derive(Clone, Debug, Default)]
pub struct PgArchiveCleanupBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    debug: bool,
    dry_run: bool,
    version: bool,
    ext: Option<OsString>,
    help: bool,
    archive_location: Option<OsString>,
    oldest_kept_wal_file: Option<OsString>,
}

impl PgArchiveCleanupBuilder {
    /// Create a new [`PgArchiveCleanupBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgArchiveCleanupBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new().program_dir(settings.get_binary_dir())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// generate debug output (verbose mode)
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    /// dry run, show the names of the files that would be removed
    #[must_use]
    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }

    /// output version information, then exit
    #[must_use]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// clean up files if they have this extension
    #[must_use]
    pub fn ext<S: AsRef<OsStr>>(mut self, ext: S) -> Self {
        self.ext = Some(ext.as_ref().to_os_string());
        self
    }

    /// show help, then exit
    #[must_use]
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }

    /// archive location
    #[must_use]
    pub fn archive_location<S: AsRef<OsStr>>(mut self, archive_location: S) -> Self {
        self.archive_location = Some(archive_location.as_ref().to_os_string());
        self
    }

    /// oldest kept WAL file
    #[must_use]
    pub fn oldest_kept_wal_file<S: AsRef<OsStr>>(mut self, oldest_kept_wal_file: S) -> Self {
        self.oldest_kept_wal_file = Some(oldest_kept_wal_file.as_ref().to_os_string());
        self
    }
}

impl CommandBuilder for PgArchiveCleanupBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_archivecleanup".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if self.debug {
            args.push("-d".into());
        }

        if self.dry_run {
            args.push("-n".into());
        }

        if self.version {
            args.push("--version".into());
        }

        if let Some(ext) = &self.ext {
            args.push("-x".into());
            args.push(ext.into());
        }

        if self.help {
            args.push("--help".into());
        }

        if let Some(archive_location) = &self.archive_location {
            args.push(archive_location.into());
        }

        if let Some(oldest_kept_wal_file) = &self.oldest_kept_wal_file {
            args.push(oldest_kept_wal_file.into());
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
    fn test_builder_new() {
        let command = PgArchiveCleanupBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_archivecleanup"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgArchiveCleanupBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#""./pg_archivecleanup""#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\pg_archivecleanup""#;

        assert_eq!(format!("{command_prefix}"), command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = PgArchiveCleanupBuilder::new()
            .env("PGDATABASE", "database")
            .debug()
            .dry_run()
            .version()
            .ext("partial")
            .help()
            .archive_location("archive_location")
            .oldest_kept_wal_file("000000010000000000000001")
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(r#"{command_prefix}"pg_archivecleanup" "-d" "-n" "--version" "-x" "partial" "--help" "archive_location" "000000010000000000000001""#),
            command.to_command_string()
        );
    }
}
