use crate::Settings;
use crate::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_verifybackup` verifies a backup against the backup manifest.
#[derive(Clone, Debug, Default)]
pub struct PgVerifyBackupBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    format: Option<OsString>,
    backup_dir: Option<OsString>,
    exit_on_error: bool,
    ignore: Vec<OsString>,
    manifest_path: Option<OsString>,
    no_parse_wal: bool,
    progress: bool,
    quiet: bool,
    skip_checksums: bool,
    wal_directory: Option<OsString>,
    version: bool,
    help: bool,
}

impl PgVerifyBackupBuilder {
    /// Create a new [`PgVerifyBackupBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgVerifyBackupBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new().program_dir(settings.get_binary_dir())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// exit immediately on error
    #[must_use]
    pub fn exit_on_error(mut self) -> Self {
        self.exit_on_error = true;
        self
    }

    /// ignore indicated path
    /// May be called multiple times; each value is retained.
    #[must_use]
    pub fn ignore<S: AsRef<OsStr>>(mut self, ignore: S) -> Self {
        self.ignore.push(ignore.as_ref().to_os_string());
        self
    }

    /// use specified path for manifest
    #[must_use]
    pub fn manifest_path<S: AsRef<OsStr>>(mut self, manifest_path: S) -> Self {
        self.manifest_path = Some(manifest_path.as_ref().to_os_string());
        self
    }

    /// do not try to parse WAL files
    #[must_use]
    pub fn no_parse_wal(mut self) -> Self {
        self.no_parse_wal = true;
        self
    }

    /// show progress information
    #[must_use]
    pub fn progress(mut self) -> Self {
        self.progress = true;
        self
    }

    /// do not print any output, except for errors
    #[must_use]
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// skip checksum verification
    #[must_use]
    pub fn skip_checksums(mut self) -> Self {
        self.skip_checksums = true;
        self
    }

    /// use specified path for WAL files
    #[must_use]
    pub fn wal_directory<S: AsRef<OsStr>>(mut self, wal_directory: S) -> Self {
        self.wal_directory = Some(wal_directory.as_ref().to_os_string());
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

    /// Select the backup format: plain or tar.
    #[must_use]
    pub fn format<S: AsRef<OsStr>>(mut self, format: S) -> Self {
        self.format = Some(format.as_ref().to_os_string());
        self
    }

    /// Directory containing the backup to verify.
    #[must_use]
    pub fn backup_dir<S: AsRef<OsStr>>(mut self, backup_dir: S) -> Self {
        self.backup_dir = Some(backup_dir.as_ref().to_os_string());
        self
    }
}

impl CommandBuilder for PgVerifyBackupBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_verifybackup".as_ref()
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

        if self.exit_on_error {
            args.push("--exit-on-error".into());
        }

        for ignore in &self.ignore {
            args.push("--ignore".into());
            args.push(ignore.into());
        }

        if let Some(manifest_path) = &self.manifest_path {
            args.push("--manifest-path".into());
            args.push(manifest_path.into());
        }

        if self.no_parse_wal {
            args.push("--no-parse-wal".into());
        }

        if self.progress {
            args.push("--progress".into());
        }

        if self.quiet {
            args.push("--quiet".into());
        }

        if self.skip_checksums {
            args.push("--skip-checksums".into());
        }

        if let Some(wal_directory) = &self.wal_directory {
            args.push("--wal-directory".into());
            args.push(wal_directory.into());
        }

        if let Some(format) = &self.format {
            args.push("--format".into());
            args.push(format.into());
        }

        if let Some(backup_dir) = &self.backup_dir {
            args.push(backup_dir.into());
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
        let command = PgVerifyBackupBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_verifybackup"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgVerifyBackupBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#""./pg_verifybackup""#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\pg_verifybackup""#;

        assert_eq!(format!("{command_prefix}"), command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = PgVerifyBackupBuilder::new()
            .env("PGDATABASE", "database")
            .exit_on_error()
            .ignore("ignore")
            .manifest_path("manifest-path")
            .no_parse_wal()
            .progress()
            .quiet()
            .skip_checksums()
            .wal_directory("wal_directory")
            .version()
            .help()
            .format("tar")
            .backup_dir("backup")
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"pg_verifybackup" "--version" "--help" "--exit-on-error" "--ignore" "ignore" "--manifest-path" "manifest-path" "--no-parse-wal" "--progress" "--quiet" "--skip-checksums" "--wal-directory" "wal_directory" "--format" "tar" "backup""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_positional_arguments_follow_options() {
        assert_args(
            PgVerifyBackupBuilder::new()
                .backup_dir("backup dir")
                .no_parse_wal(),
            &["--no-parse-wal", "backup dir"],
        );
    }

    #[test]
    fn test_options() {
        assert_args(
            PgVerifyBackupBuilder::new().format("tar"),
            &["--format", "tar"],
        );
        assert_args(
            PgVerifyBackupBuilder::new().backup_dir("value with spaces"),
            &["value with spaces"],
        );
    }

    #[test]
    fn test_repeated_options() {
        assert_args(
            PgVerifyBackupBuilder::new()
                .ignore("first")
                .ignore("second"),
            &["--ignore", "first", "--ignore", "second"],
        );
    }

    #[test]
    fn test_informational_options_come_first_on_configured_builders() {
        let version = PgVerifyBackupBuilder::from(&TestSettings)
            .version()
            .get_args();
        assert_eq!(version.first(), Some(&OsString::from("--version")));
        let help = PgVerifyBackupBuilder::from(&TestSettings).help().get_args();
        assert_eq!(help.first(), Some(&OsString::from("--help")));
    }

    #[test]
    fn test_cli_help_and_version() -> TestResult {
        let Some(bin) = binary_dir() else {
            return Ok(());
        };
        PgVerifyBackupBuilder::new()
            .program_dir(&bin)
            .version()
            .build()
            .execute()?;
        PgVerifyBackupBuilder::new()
            .program_dir(&bin)
            .help()
            .build()
            .execute()?;
        Ok(())
    }
}
