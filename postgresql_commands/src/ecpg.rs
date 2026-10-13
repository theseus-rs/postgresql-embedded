use crate::Settings;
use crate::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `ecpg` is the `PostgreSQL` embedded SQL preprocessor for C programs.
#[derive(Clone, Debug, Default)]
pub struct EcpgBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    debug: bool,
    verbose: bool,
    file: Vec<OsString>,
    c: bool,
    compatibility_mode: Option<OsString>,
    symbol: Vec<OsString>,
    header_file: bool,
    system_include_files: bool,
    directory: Vec<OsString>,
    outfile: Option<OsString>,
    runtime_behavior: Vec<OsString>,
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
    /// May be called multiple times; each value is retained.
    #[must_use]
    pub fn symbol<S: AsRef<OsStr>>(mut self, symbol: S) -> Self {
        self.symbol.push(symbol.as_ref().to_os_string());
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
    /// May be called multiple times; each value is retained.
    #[must_use]
    pub fn directory<S: AsRef<OsStr>>(mut self, directory: S) -> Self {
        self.directory.push(directory.as_ref().to_os_string());
        self
    }

    /// Write result to OUTFILE
    #[must_use]
    pub fn outfile<S: AsRef<OsStr>>(mut self, outfile: S) -> Self {
        self.outfile = Some(outfile.as_ref().to_os_string());
        self
    }

    /// Specify run-time behavior
    /// May be called multiple times; each value is retained.
    #[must_use]
    pub fn runtime_behavior<S: AsRef<OsStr>>(mut self, runtime_behavior: S) -> Self {
        self.runtime_behavior
            .push(runtime_behavior.as_ref().to_os_string());
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

    /// Enable parser debugging, when compiled into ecpg.
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    /// Print the preprocessor version and include search paths.
    #[must_use]
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// Add an input source file. May be called multiple times.
    #[must_use]
    pub fn file<S: AsRef<OsStr>>(mut self, file: S) -> Self {
        self.file.push(file.as_ref().to_os_string());
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

        if self.version {
            args.push("--version".into());
        }

        if self.help {
            args.push("--help".into());
        }

        if self.c {
            args.push("-c".into());
        }

        if let Some(mode) = &self.compatibility_mode {
            args.push("-C".into());
            args.push(mode.into());
        }

        for symbol in &self.symbol {
            args.push("-D".into());
            args.push(symbol.into());
        }

        if self.header_file {
            args.push("-h".into());
        }

        if self.system_include_files {
            args.push("-i".into());
        }

        for directory in &self.directory {
            args.push("-I".into());
            args.push(directory.into());
        }

        if let Some(outfile) = &self.outfile {
            args.push("-o".into());
            args.push(outfile.into());
        }

        for behavior in &self.runtime_behavior {
            args.push("-r".into());
            args.push(behavior.into());
        }

        if self.regression {
            args.push("--regression".into());
        }

        if self.autocommit {
            args.push("-t".into());
        }

        if self.debug {
            args.push("-d".into());
        }

        if self.verbose {
            args.push("-v".into());
        }

        for file in &self.file {
            args.push(file.into());
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
        let command_prefix = r#""./ecpg""#;
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
            .debug()
            .verbose()
            .file("input.pgc")
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"ecpg" "--version" "--help" "-c" "-C" "mode" "-D" "symbol" "-h" "-i" "-I" "directory" "-o" "outfile" "-r" "behavior" "--regression" "-t" "-d" "-v" "input.pgc""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_positional_arguments_follow_options() {
        assert_args(
            EcpgBuilder::new()
                .file("one.pgc")
                .outfile("out.c")
                .file("two.pgc"),
            &["-o", "out.c", "one.pgc", "two.pgc"],
        );
    }

    #[test]
    fn test_options() {
        assert_args(EcpgBuilder::new().debug(), &["-d"]);
        assert_args(EcpgBuilder::new().verbose(), &["-v"]);
        assert_args(
            EcpgBuilder::new().file("value with spaces"),
            &["value with spaces"],
        );
    }

    #[test]
    fn test_repeated_options() {
        assert_args(
            EcpgBuilder::new().symbol("first").symbol("second"),
            &["-D", "first", "-D", "second"],
        );
        assert_args(
            EcpgBuilder::new().directory("first").directory("second"),
            &["-I", "first", "-I", "second"],
        );
        assert_args(
            EcpgBuilder::new()
                .runtime_behavior("first")
                .runtime_behavior("second"),
            &["-r", "first", "-r", "second"],
        );
    }

    #[test]
    fn test_informational_options_come_first_on_configured_builders() {
        let version = EcpgBuilder::from(&TestSettings).version().get_args();
        assert_eq!(version.first(), Some(&OsString::from("--version")));
        let help = EcpgBuilder::from(&TestSettings).help().get_args();
        assert_eq!(help.first(), Some(&OsString::from("--help")));
    }

    #[cfg(unix)]
    #[test]
    fn test_native_argument_bytes_are_preserved() {
        use std::os::unix::ffi::OsStringExt;

        let value = OsString::from_vec(b"value\xfe".to_vec());
        assert_eq!(EcpgBuilder::new().file(&value).get_args(), vec![value]);
    }

    #[test]
    fn test_cli_help_and_version() -> TestResult {
        let Some(bin) = binary_dir() else {
            return Ok(());
        };
        EcpgBuilder::new()
            .program_dir(&bin)
            .version()
            .build()
            .execute()?;
        EcpgBuilder::new()
            .program_dir(&bin)
            .help()
            .build()
            .execute()?;
        Ok(())
    }
}
