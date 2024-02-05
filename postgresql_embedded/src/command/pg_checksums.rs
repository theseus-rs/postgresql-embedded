use crate::command::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// pg_checksums enables, disables, or verifies data checksums in a PostgreSQL database cluster.
#[derive(Clone, Debug, Default)]
pub struct PgChecksumsBuilder {
    program_dir: Option<PathBuf>,
    pgdata: Option<PathBuf>,
    check: bool,
    disable: bool,
    enable: bool,
    filenode: Option<OsString>,
    no_sync: bool,
    progress: bool,
    verbose: bool,
    version: bool,
    help: bool,
}

impl PgChecksumsBuilder {
    /// Create a new [`PgChecksumsBuilder`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Location of the program binary
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// data directory
    pub fn pgdata<P: Into<PathBuf>>(mut self, pgdata: P) -> Self {
        self.pgdata = Some(pgdata.into());
        self
    }

    /// check data checksums (default)
    pub fn check(mut self) -> Self {
        self.check = true;
        self
    }

    /// disable data checksums
    pub fn disable(mut self) -> Self {
        self.disable = true;
        self
    }

    /// enable data checksums
    pub fn enable(mut self) -> Self {
        self.enable = true;
        self
    }

    /// check only relation with specified filenode
    pub fn filenode<S: AsRef<OsStr>>(mut self, filenode: S) -> Self {
        self.filenode = Some(filenode.as_ref().to_os_string());
        self
    }

    /// do not wait for changes to be written safely to disk
    pub fn no_sync(mut self) -> Self {
        self.no_sync = true;
        self
    }

    /// show progress information
    pub fn progress(mut self) -> Self {
        self.progress = true;
        self
    }

    /// output verbose messages
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
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

impl CommandBuilder for PgChecksumsBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_checksums".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(pgdata) = &self.pgdata {
            args.push("--pgdata".into());
            args.push(pgdata.into());
        }

        if self.check {
            args.push("--check".into());
        }

        if self.disable {
            args.push("--disable".into());
        }

        if self.enable {
            args.push("--enable".into());
        }

        if let Some(filenode) = &self.filenode {
            args.push("--filenode".into());
            args.push(filenode.into());
        }

        if self.no_sync {
            args.push("--no-sync".into());
        }

        if self.progress {
            args.push("--progress".into());
        }

        if self.verbose {
            args.push("--verbose".into());
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
        let command = PgChecksumsBuilder::new().build();

        assert_eq!(r#""pg_checksums""#, command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = PgChecksumsBuilder::new()
            .program_dir("/usr/bin")
            .pgdata("/path/to/pgdata")
            .check()
            .disable()
            .enable()
            .filenode("12345")
            .no_sync()
            .progress()
            .verbose()
            .version()
            .help()
            .build();

        assert_eq!(
            r#""/usr/bin/pg_checksums" "--pgdata" "/path/to/pgdata" "--check" "--disable" "--enable" "--filenode" "12345" "--no-sync" "--progress" "--verbose" "--version" "--help""#,
            command.to_command_string()
        );
    }
}
