use crate::command::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// pg_verifybackup verifies a backup against the backup manifest.
#[derive(Clone, Debug, Default)]
pub struct PgVerifyBackupBuilder {
    program_dir: Option<PathBuf>,
    exit_on_error: bool,
    ignore: Option<OsString>,
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
    pub fn new() -> Self {
        Self::default()
    }

    /// Location of the program binary
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// exit immediately on error
    pub fn exit_on_error(mut self) -> Self {
        self.exit_on_error = true;
        self
    }

    /// ignore indicated path
    pub fn ignore<S: AsRef<OsStr>>(mut self, ignore: S) -> Self {
        self.ignore = Some(ignore.as_ref().to_os_string());
        self
    }

    /// use specified path for manifest
    pub fn manifest_path<S: AsRef<OsStr>>(mut self, manifest_path: S) -> Self {
        self.manifest_path = Some(manifest_path.as_ref().to_os_string());
        self
    }

    /// do not try to parse WAL files
    pub fn no_parse_wal(mut self) -> Self {
        self.no_parse_wal = true;
        self
    }

    /// show progress information
    pub fn progress(mut self) -> Self {
        self.progress = true;
        self
    }

    /// do not print any output, except for errors
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// skip checksum verification
    pub fn skip_checksums(mut self) -> Self {
        self.skip_checksums = true;
        self
    }

    /// use specified path for WAL files
    pub fn wal_directory<S: AsRef<OsStr>>(mut self, wal_directory: S) -> Self {
        self.wal_directory = Some(wal_directory.as_ref().to_os_string());
        self
    }

    /// output version information, then exit
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// show help, then exit
    pub fn help(mut self) -> Self {
        self.help = true;
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

        if self.exit_on_error {
            args.push("--exit-on-error".into());
        }

        if let Some(ignore) = &self.ignore {
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

        if self.version {
            args.push("--version".into());
        }

        if self.help {
            args.push("--help".into());
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
        let command = PgVerifyBackupBuilder::new().program_dir(".").build();

        assert_eq!(
            PathBuf::from(".").join("pg_verifybackup"),
            PathBuf::from(command.to_command_string().replace("\"", ""))
        );
    }

    #[test]
    fn test_builder() {
        let command = PgVerifyBackupBuilder::new()
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
            .build();

        assert_eq!(
            r#""pg_verifybackup" "--exit-on-error" "--ignore" "ignore" "--manifest-path" "manifest-path" "--no-parse-wal" "--progress" "--quiet" "--skip-checksums" "--wal-directory" "wal_directory" "--version" "--help""#,
            command.to_command_string()
        );
    }
}
