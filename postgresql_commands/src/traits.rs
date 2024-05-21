use crate::error::{Error, Result};
use std::ffi::{OsStr, OsString};
use std::fmt::Debug;
use std::path::PathBuf;
use std::time::Duration;
use tracing::debug;

/// Interface for PostgreSQL settings
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
    fn get_envs(&self) -> Vec<(OsString, OsString)> {
        vec![]
    }

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
}

#[cfg(feature = "tokio")]
/// Implement the [`CommandExecutor`] trait for [`Command`](tokio::process::Command)
impl AsyncCommandExecutor for tokio::process::Command {
    /// Execute the command and return the stdout and stderr
    async fn execute(&mut self, timeout: Option<Duration>) -> Result<(String, String)> {
        #[tracing::instrument(level = "debug", skip(reader))]
        async fn read_process(
            mut reader: Option<impl tokio::io::AsyncRead + Unpin>,
            mut exit_anyway_broadcast_receiver: tokio::sync::broadcast::Receiver<()>,
        ) -> Result<String> {
            let Some(reader) = reader.as_mut() else {
                return Ok(String::new());
            };
            let mut vec = Vec::new();
            loop {
                use tokio::io::AsyncReadExt as _;
                tokio::select! {
                    n = reader.read_buf(&mut vec) => {
                        if n? == 0 {
                            return Ok(String::from_utf8_lossy(&*vec).into_owned());
                        }
                    },
                    _ = exit_anyway_broadcast_receiver.recv() => {
                        return Ok(String::from_utf8_lossy(&*vec).into_owned());
                    },
                }
            }
        }

        debug!("Executing command: {}", self.to_command_string());

        let res_fut = async {
            let mut child = self
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()?;

            let stdout = child.stdout.take();
            let stderr = child.stderr.take();

            // on windows, pg_ctl start will appear to hang if you try to read out all of stdout
            // and stderr. so, on windows do a horrible hack and forcibly end reading of stdout
            // and stderr 50ms after the process exits. on not-windows, this early exit mechanism
            //  is set up but never sent to, resulting in the same behavior as `read_to_end`.

            let (exit_anyway_broadcast_sender, exit_anyway_broadcast_receiver_stdout) =
                tokio::sync::broadcast::channel(1);
            let exit_anyway_broadcast_receiver_stderr = exit_anyway_broadcast_sender.subscribe();
            let stdout = tokio::spawn(async {
                read_process(stdout, exit_anyway_broadcast_receiver_stdout).await
            });
            let stderr = tokio::spawn(async {
                read_process(stderr, exit_anyway_broadcast_receiver_stderr).await
            });
            let exit_status = child.wait().await;
            #[cfg(target_os = "windows")]
            {
                tokio::time::sleep(Duration::from_millis(50)).await;
                let _ = exit_anyway_broadcast_sender.send(());
            }
            let (stdout, stderr) = tokio::join!(stdout, stderr);
            std::mem::drop(exit_anyway_broadcast_sender);

            let exit_status = exit_status?;
            fn debug_render(
                which: &'static str,
                res: &std::result::Result<Result<String>, tokio::task::JoinError>,
            ) -> String {
                match res {
                    Ok(Ok(s)) => s.into(),
                    Ok(Err(io_err)) => format!("<failed to read {}: {:?}>", which, io_err),
                    Err(join_err) => format!("<failed to read {}: {:?}>", which, join_err),
                }
            }
            debug!(
                "Result: {}\nstdout: {}\nstderr: {}",
                exit_status
                    .code()
                    .map_or("None".to_string(), |c| c.to_string()),
                debug_render("stdout", &stdout),
                debug_render("stderr", &stderr)
            );

            fn unwrap2_or_empty_string<E, E2>(
                r: std::result::Result<std::result::Result<String, E>, E2>,
            ) -> String {
                r.map_or_else(|_| String::new(), |r| r.unwrap_or_else(|_| String::new()))
            }

            let stdout = unwrap2_or_empty_string(stdout);
            let stderr = unwrap2_or_empty_string(stderr);

            if exit_status.success() {
                Ok((stdout, stderr))
            } else {
                Err(Error::CommandError { stdout, stderr })
            }
        };

        match timeout {
            Some(duration) => tokio::time::timeout(duration, res_fut).await?,
            None => res_fut.await,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_log::test;

    #[test]
    fn test_command_builder_defaults() {
        #[derive(Debug)]
        struct DefaultCommandBuilder {
            program_dir: Option<PathBuf>,
        }

        impl CommandBuilder for DefaultCommandBuilder {
            fn get_program(&self) -> &'static OsStr {
                "test".as_ref()
            }

            fn get_program_dir(&self) -> &Option<PathBuf> {
                &self.program_dir
            }
        }

        let builder = DefaultCommandBuilder { program_dir: None };
        let command = builder.build();

        assert_eq!(r#""test""#, command.to_command_string());
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
    }

    #[test]
    fn test_standard_command_builder() {
        let builder = TestCommandBuilder {
            program_dir: None,
            args: vec!["--help".to_string().into()],
            envs: vec![(OsString::from("PASSWORD"), OsString::from("foo"))],
        };
        let command = builder.build();

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
            envs: vec![(OsString::from("PASSWORD"), OsString::from("foo"))],
        };
        let command = builder.build_tokio();

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
}
