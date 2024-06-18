use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_test_fsync` command to determine fastest `wal_sync_method` for `PostgreSQL`
#[derive(Clone, Debug, Default)]
#[allow(clippy::module_name_repetitions)]
pub struct PgTestFsyncBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    filename: Option<OsString>,
    secs_per_test: Option<usize>,
}

impl PgTestFsyncBuilder {
    /// Create a new [`PgTestFsyncBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgTestFsyncBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new().program_dir(settings.get_binary_dir())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// Set the filename
    #[must_use]
    pub fn filename<S: AsRef<OsStr>>(mut self, filename: S) -> Self {
        self.filename = Some(filename.as_ref().to_os_string());
        self
    }

    /// Set the seconds per test
    #[must_use]
    pub fn secs_per_test(mut self, secs: usize) -> Self {
        self.secs_per_test = Some(secs);
        self
    }
}

impl CommandBuilder for PgTestFsyncBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_test_fsync".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(filename) = &self.filename {
            args.push("-f".into());
            args.push(filename.into());
        }

        if let Some(secs) = &self.secs_per_test {
            args.push("-s".into());
            args.push(secs.to_string().into());
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
        let command = PgTestFsyncBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_test_fsync"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgTestFsyncBuilder::from(&TestSettings).build();
        assert_eq!(r#""./pg_test_fsync""#, command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = PgTestFsyncBuilder::new()
            .env("PGDATABASE", "database")
            .filename("filename")
            .secs_per_test(10)
            .build();

        assert_eq!(
            r#"PGDATABASE="database" "pg_test_fsync" "-f" "filename" "-s" "10""#,
            command.to_command_string()
        );
    }
}
