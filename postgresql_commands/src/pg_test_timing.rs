use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// pg_test_timing tests the timing of a PostgreSQL instance.
#[derive(Clone, Debug, Default)]
pub struct PgTestTimingBuilder {
    program_dir: Option<PathBuf>,
    duration: Option<OsString>,
}

impl PgTestTimingBuilder {
    /// Create a new [PgTestTimingBuilder]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [PgTestTimingBuilder] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new().program_dir(settings.get_binary_dir())
    }

    /// Location of the program binary
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// set the duration for the test
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::CommandToString;
    use crate::TestSettings;
    use test_log::test;

    #[test]
    fn test_builder_new() {
        let command = PgTestTimingBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_test_timing"),
            PathBuf::from(command.to_command_string().replace("\"", ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgTestTimingBuilder::from(&TestSettings).build();
        assert_eq!(r#""./pg_test_timing""#, command.to_command_string())
    }

    #[test]
    fn test_builder() {
        let command = PgTestTimingBuilder::new().duration("10").build();

        assert_eq!(r#""pg_test_timing" "-d" "10""#, command.to_command_string());
    }
}
