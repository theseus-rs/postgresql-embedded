use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `ecpg` is the `PostgreSQL` embedded SQL preprocessor for C programs.
#[derive(Clone, Debug, Default)]
pub struct EcpgBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    c: bool,
    compatibility_mode: Option<OsString>,
    symbol: Option<OsString>,
    header_file: bool,
    system_include_files: bool,
    directory: Option<OsString>,
    outfile: Option<OsString>,
    runtime_behavior: Option<OsString>,
    regression: bool,
    autocommit: bool,
    version: bool,
    help: bool,
}

impl EcpgBuilder {
    /// Create a new [`EcpgBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`EcpgBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new().program_dir(settings.get_binary_dir())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// Automatically generate C code from embedded SQL code
    #[must_use]
    pub fn c(mut self) -> Self {
        self.c = true;
        self
    }

    /// Set compatibility mode
    #[must_use]
    pub fn compatibility_mode<S: AsRef<OsStr>>(mut self, compatibility_mode: S) -> Self {
        self.compatibility_mode = Some(compatibility_mode.as_ref().to_os_string());
        self
    }

    /// Define SYMBOL
    #[must_use]
    pub fn symbol<S: AsRef<OsStr>>(mut self, symbol: S) -> Self {
        self.symbol = Some(symbol.as_ref().to_os_string());
        self
    }

    /// Parse a header file
    #[must_use]
    pub fn header_file(mut self) -> Self {
        self.header_file = true;
        self.c()
    }

    /// Parse system include files as well
    #[must_use]
    pub fn system_include_files(mut self) -> Self {
        self.system_include_files = true;
        self
    }

    /// Search DIRECTORY for include files
    #[must_use]
    pub fn directory<S: AsRef<OsStr>>(mut self, directory: S) -> Self {
        self.directory = Some(directory.as_ref().to_os_string());
        self
    }

    /// Write result to OUTFILE
    #[must_use]
    pub fn outfile<S: AsRef<OsStr>>(mut self, outfile: S) -> Self {
        self.outfile = Some(outfile.as_ref().to_os_string());
        self
    }

    /// Specify run-time behavior
    #[must_use]
    pub fn runtime_behavior<S: AsRef<OsStr>>(mut self, runtime_behavior: S) -> Self {
        self.runtime_behavior = Some(runtime_behavior.as_ref().to_os_string());
        self
    }

    /// Run in regression testing mode
    #[must_use]
    pub fn regression(mut self) -> Self {
        self.regression = true;
        self
    }

    /// Turn on autocommit of transactions
    #[must_use]
    pub fn autocommit(mut self) -> Self {
        self.autocommit = true;
        self
    }

    /// Output version information, then exit
    #[must_use]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// Show help, then exit
    #[must_use]
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }
}

impl CommandBuilder for EcpgBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "ecpg".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if self.c {
            args.push("-c".into());
        }

        if let Some(mode) = &self.compatibility_mode {
            args.push("-C".into());
            args.push(mode.into());
        }

        if let Some(symbol) = &self.symbol {
            args.push("-D".into());
            args.push(symbol.into());
        }

        if self.header_file {
            args.push("-h".into());
        }

        if self.system_include_files {
            args.push("-i".into());
        }

        if let Some(directory) = &self.directory {
            args.push("-I".into());
            args.push(directory.into());
        }

        if let Some(outfile) = &self.outfile {
            args.push("-o".into());
            args.push(outfile.into());
        }

        if let Some(behavior) = &self.runtime_behavior {
            args.push("-r".into());
            args.push(behavior.into());
        }

        if self.regression {
            args.push("--regression".into());
        }

        if self.autocommit {
            args.push("-t".into());
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
    use crate::traits::CommandToString;
    use crate::TestSettings;
    use test_log::test;

    #[test]
    fn test_builder_new() {
        let command = EcpgBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("ecpg"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = EcpgBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGPASSWORD="password" "./ecpg""#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\ecpg""#;

        assert_eq!(format!("{command_prefix}"), command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = EcpgBuilder::new()
            .env("PGDATABASE", "database")
            .c()
            .compatibility_mode("mode")
            .symbol("symbol")
            .header_file()
            .system_include_files()
            .directory("directory")
            .outfile("outfile")
            .runtime_behavior("behavior")
            .regression()
            .autocommit()
            .version()
            .help()
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(r#"{command_prefix}"ecpg" "-c" "-C" "mode" "-D" "symbol" "-h" "-i" "-I" "directory" "-o" "outfile" "-r" "behavior" "--regression" "-t" "--version" "--help""#),
            command.to_command_string()
        );
    }
}
