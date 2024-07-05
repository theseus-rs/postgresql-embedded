use crate::error::{Error, Result};
use std::ffi::{OsStr, OsString};
use std::fmt::Debug;
use std::path::PathBuf;
use std::time::Duration;
use tracing::debug;

/// Interface for `PostgreSQL` settings
pub trait Settings {
    fn get_binary_dir(&self) -> PathBuf;
    fn get_host(&self) -> OsString;
    fn get_port(&self) -> u16;
    fn get_username(&self) -> OsString;
    fn get_password(&self) -> OsString;
}

#[cfg(test)]
pub struct TestSettings;

#[cfg(test)]
impl Settings for TestSettings {
    fn get_binary_dir(&self) -> PathBuf {
        PathBuf::from(".")
    }

    fn get_host(&self) -> OsString {
        "localhost".into()
    }

    fn get_port(&self) -> u16 {
        5432
    }

    fn get_username(&self) -> OsString {
        "postgres".into()
    }

    fn get_password(&self) -> OsString {
        "password".into()
    }
}

/// Trait to build a command
pub trait CommandBuilder: Debug {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr;

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf>;

    /// Fully qualified path to the program binary
    fn get_program_file(&self) -> PathBuf {
        let program_name = &self.get_program();
        match self.get_program_dir() {
            Some(program_dir) => program_dir.join(program_name),
            None => PathBuf::from(program_name),
        }
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        vec![]
    }

    /// Get the environment variables for the command
    fn get_envs(&self) -> Vec<(OsString, OsString)>;

    /// Set an environment variable for the command
    #[must_use]
    fn env<S: AsRef<OsStr>>(self, key: S, value: S) -> Self;

    /// Build a standard Command
    fn build(self) -> std::process::Command
    where
        Self: Sized,
    {
        let program_file = self.get_program_file();
        let mut command = std::process::Command::new(program_file);

        command.args(self.get_args());
        command.envs(self.get_envs());
        command
    }

    #[cfg(feature = "tokio")]
    /// Build a tokio Command
    fn build_tokio(self) -> tokio::process::Command
    where
        Self: Sized,
    {
        let program_file = self.get_program_file();
        let mut command = tokio::process::Command::new(program_file);

        command.args(self.get_args());
        command.envs(self.get_envs());
        command
    }
}

/// Trait to convert a command to a string representation
pub trait CommandToString {
    fn to_command_string(&self) -> String;
}

/// Implement the [`CommandToString`] trait for [`Command`](std::process::Command)
impl CommandToString for std::process::Command {
    fn to_command_string(&self) -> String {
        format!("{self:?}")
    }
}

#[cfg(feature = "tokio")]
/// Implement the [`CommandToString`] trait for [`Command`](tokio::process::Command)
impl CommandToString for tokio::process::Command {
    fn to_command_string(&self) -> String {
        format!("{self:?}")
            .replace("Command { std: ", "")
            .replace(", kill_on_drop: false }", "")
    }
}

/// Interface for executing a command
pub trait CommandExecutor {
    /// Execute the command and return the stdout and stderr
    ///
    /// # Errors
    ///
    /// Returns an error if the command fails
    fn execute(&mut self) -> Result<(String, String)>;
}

/// Interface for executing a command
pub trait AsyncCommandExecutor {
    /// Execute the command and return the stdout and stderr
    async fn execute(&mut self, timeout: Option<Duration>) -> Result<(String, String)>;
}

/// Implement the [`CommandExecutor`] trait for [`Command`](std::process::Command)
impl CommandExecutor for std::process::Command {
    /// Execute the command and return the stdout and stderr
    fn execute(&mut self) -> Result<(String, String)> {
        debug!("Executing command: {}", self.to_command_string());
        #[cfg(not(target_os = "windows"))]
        {
            let output = self.output()?;
            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

            debug!(
                "Result: {}\nstdout: {}\nstderr: {}",
                output
                    .status
                    .code()
                    .map_or("None".to_string(), |c| c.to_string()),
                stdout,
                stderr
            );

            if output.status.success() {
                Ok((stdout, stderr))
            } else {
                Err(Error::CommandError { stdout, stderr })
            }
        }

        // TODO: Processes can hang on Windows when attempting to get stdout/stderr using code
        // that works for Linux/MacOS; this implementation should be updated to retrieve the
        // values of stdout/stderr without hanging
        #[cfg(target_os = "windows")]
        {
            let mut process = self
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()?;
            let status = process.wait()?;
            let stdout = String::new();
            let stderr = String::new();
            if status.success() {
                Ok((stdout, stderr))
            } else {
                Err(Error::CommandError { stdout, stderr })
            }
        }
    }
}

#[cfg(feature = "tokio")]
/// Implement the [`CommandExecutor`] trait for [`Command`](tokio::process::Command)
impl AsyncCommandExecutor for tokio::process::Command {
    /// Execute the command and return the stdout and stderr
    async fn execute(&mut self, timeout: Option<Duration>) -> Result<(String, String)> {
        debug!("Executing command: {}", self.to_command_string());
        let output = match timeout {
            Some(duration) => tokio::time::timeout(duration, self.output()).await?,
            None => self.output().await,
        }?;

        #[cfg(not(target_os = "windows"))]
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        #[cfg(not(target_os = "windows"))]
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

        // TODO: Processes can hang on Windows when attempting to get stdout/stderr using code
        // that works for Linux/MacOS; this implementation should be updated to retrieve the
        // values of stdout/stderr without hanging
        #[cfg(target_os = "windows")]
        let stdout = String::new();
        #[cfg(target_os = "windows")]
        let stderr = String::new();

        debug!(
            "Result: {}\nstdout: {}\nstderr: {}",
            output
                .status
                .code()
                .map_or("None".to_string(), |c| c.to_string()),
            stdout,
            stderr
        );

        if output.status.success() {
            Ok((stdout, stderr))
        } else {
            Err(Error::CommandError { stdout, stderr })
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use test_log::test;

    #[test]
    fn test_command_builder_defaults() {
        #[derive(Debug, Default)]
        struct DefaultCommandBuilder {
            program_dir: Option<PathBuf>,
            envs: Vec<(OsString, OsString)>,
        }

        impl CommandBuilder for DefaultCommandBuilder {
            fn get_program(&self) -> &'static OsStr {
                "test".as_ref()
            }

            fn get_program_dir(&self) -> &Option<PathBuf> {
                &self.program_dir
            }

            fn get_envs(&self) -> Vec<(OsString, OsString)> {
                self.envs.clone()
            }

            fn env<S: AsRef<OsStr>>(mut self, key: S, value: S) -> Self {
                self.envs
                    .push((key.as_ref().to_os_string(), value.as_ref().to_os_string()));
                self
            }
        }

        let builder = DefaultCommandBuilder::default();
        let command = builder.env("ENV", "foo").build();

        assert_eq!(r#"ENV="foo" "test""#, command.to_command_string());
    }

    #[derive(Debug)]
    struct TestCommandBuilder {
        program_dir: Option<PathBuf>,
        args: Vec<OsString>,
        envs: Vec<(OsString, OsString)>,
    }

    impl CommandBuilder for TestCommandBuilder {
        fn get_program(&self) -> &'static OsStr {
            "test".as_ref()
        }

        fn get_program_dir(&self) -> &Option<PathBuf> {
            &self.program_dir
        }

        fn get_args(&self) -> Vec<OsString> {
            self.args.clone()
        }

        fn get_envs(&self) -> Vec<(OsString, OsString)> {
            self.envs.clone()
        }

        fn env<S: AsRef<OsStr>>(mut self, key: S, value: S) -> Self {
            self.envs
                .push((key.as_ref().to_os_string(), value.as_ref().to_os_string()));
            self
        }
    }

    #[test]
    fn test_standard_command_builder() {
        let builder = TestCommandBuilder {
            program_dir: None,
            args: vec!["--help".to_string().into()],
            envs: vec![],
        };
        let command = builder.env("PASSWORD", "foo").build();

        assert_eq!(
            format!(
                r#"PASSWORD="foo" "{}" "--help""#,
                PathBuf::from("test").to_string_lossy()
            ),
            command.to_command_string()
        );
    }

    #[cfg(feature = "tokio")]
    #[test]
    fn test_tokio_command_builder() {
        let builder = TestCommandBuilder {
            program_dir: None,
            args: vec!["--help".to_string().into()],
            envs: vec![],
        };
        let command = builder.env("PASSWORD", "foo").build_tokio();

        assert_eq!(
            format!(
                r#"PASSWORD="foo" "{}" "--help""#,
                PathBuf::from("test").to_string_lossy()
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_standard_to_command_string() {
        let mut command = std::process::Command::new("test");
        command.arg("-l");
        assert_eq!(r#""test" "-l""#, command.to_command_string(),);
    }

    #[cfg(feature = "tokio")]
    #[test]
    fn test_tokio_to_command_string() {
        let mut command = tokio::process::Command::new("test");
        command.arg("-l");
        assert_eq!(r#""test" "-l""#, command.to_command_string(),);
    }

    #[test(tokio::test)]
    async fn test_standard_command_execute() -> Result<()> {
        #[cfg(not(target_os = "windows"))]
        let mut command = std::process::Command::new("sh");
        #[cfg(not(target_os = "windows"))]
        command.args(["-c", "echo foo"]);

        #[cfg(target_os = "windows")]
        let mut command = std::process::Command::new("cmd");
        #[cfg(target_os = "windows")]
        command.args(&["/C", "echo foo"]);

        let (stdout, stderr) = command.execute()?;
        assert!(stdout.starts_with("foo"));
        assert!(stderr.is_empty());
        Ok(())
    }

    #[test(tokio::test)]
    async fn test_standard_command_execute_error() {
        let mut command = std::process::Command::new("bogus_command");
        assert!(command.execute().is_err());
    }

    #[cfg(feature = "tokio")]
    #[test(tokio::test)]
    async fn test_tokio_command_execute() -> Result<()> {
        #[cfg(not(target_os = "windows"))]
        let mut command = tokio::process::Command::new("sh");
        #[cfg(not(target_os = "windows"))]
        command.args(["-c", "echo foo"]);

        #[cfg(target_os = "windows")]
        let mut command = tokio::process::Command::new("cmd");
        #[cfg(target_os = "windows")]
        command.args(&["/C", "echo foo"]);

        let (stdout, stderr) = command.execute(None).await?;
        assert!(stdout.starts_with("foo"));
        assert!(stderr.is_empty());
        Ok(())
    }

    #[cfg(feature = "tokio")]
    #[test(tokio::test)]
    async fn test_tokio_command_execute_error() -> Result<()> {
        let mut command = tokio::process::Command::new("bogus_command");
        assert!(command.execute(None).await.is_err());
        Ok(())
    }
}
