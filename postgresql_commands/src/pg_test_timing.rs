use crate::Settings;
use crate::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_test_timing` tests the timing of a `PostgreSQL` instance.
#[derive(Clone, Debug, Default)]
pub struct PgTestTimingBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    duration: Option<OsString>,
}

impl PgTestTimingBuilder {
    /// Create a new [`PgTestTimingBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgTestTimingBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new().program_dir(settings.get_binary_dir())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// set the duration for the test
    #[must_use]
    pub fn duration<S: AsRef<OsStr>>(mut self, duration: S) -> Self {
        self.duration = Some(duration.as_ref().to_os_string());
        self
    }
}

impl CommandBuilder for PgTestTimingBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_test_timing".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(duration) = &self.duration {
            args.push("-d".into());
            args.push(duration.into());
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
        let command = PgTestTimingBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_test_timing"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgTestTimingBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#""./pg_test_timing""#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\pg_test_timing""#;

        assert_eq!(format!("{command_prefix}"), command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = PgTestTimingBuilder::new()
            .env("PGDATABASE", "database")
            .duration("10")
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(r#"{command_prefix}"pg_test_timing" "-d" "10""#),
            command.to_command_string()
        );
    }
}
