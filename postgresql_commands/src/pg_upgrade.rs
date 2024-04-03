use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// pg_upgrade upgrades a PostgreSQL cluster to a different major version.
#[derive(Clone, Debug, Default)]
pub struct PgUpgradeBuilder {
    program_dir: Option<PathBuf>,
    old_bindir: Option<OsString>,
    new_bindir: Option<OsString>,
    check: bool,
    old_datadir: Option<OsString>,
    new_datadir: Option<OsString>,
    jobs: Option<OsString>,
    link: bool,
    no_sync: bool,
    old_options: Option<OsString>,
    new_options: Option<OsString>,
    old_port: Option<u16>,
    new_port: Option<u16>,
    retain: bool,
    socketdir: Option<OsString>,
    username: Option<OsString>,
    verbose: bool,
    version: bool,
    clone: bool,
    copy: bool,
    help: bool,
}

impl PgUpgradeBuilder {
    /// Create a new [PgUpgradeBuilder]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [PgUpgradeBuilder] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new().program_dir(settings.get_binary_dir())
    }

    /// Location of the program binary
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// old cluster executable directory
    pub fn old_bindir<S: AsRef<OsStr>>(mut self, old_bindir: S) -> Self {
        self.old_bindir = Some(old_bindir.as_ref().to_os_string());
        self
    }

    /// new cluster executable directory
    pub fn new_bindir<S: AsRef<OsStr>>(mut self, new_bindir: S) -> Self {
        self.new_bindir = Some(new_bindir.as_ref().to_os_string());
        self
    }

    /// check clusters only, don't change any data
    pub fn check(mut self) -> Self {
        self.check = true;
        self
    }

    /// old cluster data directory
    pub fn old_datadir<S: AsRef<OsStr>>(mut self, old_datadir: S) -> Self {
        self.old_datadir = Some(old_datadir.as_ref().to_os_string());
        self
    }

    /// new cluster data directory
    pub fn new_datadir<S: AsRef<OsStr>>(mut self, new_datadir: S) -> Self {
        self.new_datadir = Some(new_datadir.as_ref().to_os_string());
        self
    }

    /// number of simultaneous processes or threads to use
    pub fn jobs<S: AsRef<OsStr>>(mut self, jobs: S) -> Self {
        self.jobs = Some(jobs.as_ref().to_os_string());
        self
    }

    /// link instead of copying files to new cluster
    pub fn link(mut self) -> Self {
        self.link = true;
        self
    }

    /// do not wait for changes to be written safely to disk
    pub fn no_sync(mut self) -> Self {
        self.no_sync = true;
        self
    }

    /// old cluster options to pass to the server
    pub fn old_options<S: AsRef<OsStr>>(mut self, old_options: S) -> Self {
        self.old_options = Some(old_options.as_ref().to_os_string());
        self
    }

    /// new cluster options to pass to the server
    pub fn new_options<S: AsRef<OsStr>>(mut self, new_options: S) -> Self {
        self.new_options = Some(new_options.as_ref().to_os_string());
        self
    }

    /// old cluster port number
    pub fn old_port(mut self, old_port: u16) -> Self {
        self.old_port = Some(old_port);
        self
    }

    /// new cluster port number
    pub fn new_port(mut self, new_port: u16) -> Self {
        self.new_port = Some(new_port);
        self
    }

    /// retain SQL and log files after success
    pub fn retain(mut self) -> Self {
        self.retain = true;
        self
    }

    /// socket directory to use
    pub fn socketdir<S: AsRef<OsStr>>(mut self, socketdir: S) -> Self {
        self.socketdir = Some(socketdir.as_ref().to_os_string());
        self
    }

    /// cluster superuser
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
        self
    }

    /// enable verbose internal logging
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// display version information, then exit
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// clone instead of copying files to new cluster
    pub fn clone(mut self) -> Self {
        self.clone = true;
        self
    }

    /// copy files to new cluster
    pub fn copy(mut self) -> Self {
        self.copy = true;
        self
    }

    /// show help, then exit
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }
}

impl CommandBuilder for PgUpgradeBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_upgrade".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(old_bindir) = &self.old_bindir {
            args.push("--old-bindir".into());
            args.push(old_bindir.into());
        }

        if let Some(new_bindir) = &self.new_bindir {
            args.push("--new-bindir".into());
            args.push(new_bindir.into());
        }

        if self.check {
            args.push("--check".into());
        }

        if let Some(old_datadir) = &self.old_datadir {
            args.push("--old-datadir".into());
            args.push(old_datadir.into());
        }

        if let Some(new_datadir) = &self.new_datadir {
            args.push("--new-datadir".into());
            args.push(new_datadir.into());
        }

        if let Some(jobs) = &self.jobs {
            args.push("--jobs".into());
            args.push(jobs.into());
        }

        if self.link {
            args.push("--link".into());
        }

        if self.no_sync {
            args.push("--no-sync".into());
        }

        if let Some(old_options) = &self.old_options {
            args.push("--old-options".into());
            args.push(old_options.into());
        }

        if let Some(new_options) = &self.new_options {
            args.push("--new-options".into());
            args.push(new_options.into());
        }

        if let Some(old_port) = &self.old_port {
            args.push("--old-port".into());
            args.push(old_port.to_string().into());
        }

        if let Some(new_port) = &self.new_port {
            args.push("--new-port".into());
            args.push(new_port.to_string().into());
        }

        if self.retain {
            args.push("--retain".into());
        }

        if let Some(socketdir) = &self.socketdir {
            args.push("--socketdir".into());
            args.push(socketdir.into());
        }

        if let Some(username) = &self.username {
            args.push("--username".into());
            args.push(username.into());
        }

        if self.verbose {
            args.push("--verbose".into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.clone {
            args.push("--clone".into());
        }

        if self.copy {
            args.push("--copy".into());
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
    use crate::traits::CommandToString;
    use crate::TestSettings;
    use test_log::test;

    #[test]
    fn test_builder_new() {
        let command = PgUpgradeBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_upgrade"),
            PathBuf::from(command.to_command_string().replace("\"", ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgUpgradeBuilder::from(&TestSettings).build();
        assert_eq!(r#""./pg_upgrade""#, command.to_command_string())
    }

    #[test]
    fn test_builder() {
        let command = PgUpgradeBuilder::new()
            .old_bindir("old")
            .new_bindir("new")
            .check()
            .old_datadir("old_data")
            .new_datadir("new_data")
            .jobs("10")
            .link()
            .no_sync()
            .old_options("old")
            .new_options("new")
            .old_port(5432)
            .new_port(5433)
            .retain()
            .socketdir("socket")
            .username("user")
            .verbose()
            .version()
            .clone()
            .copy()
            .help()
            .build();

        assert_eq!(
            r#""pg_upgrade" "--old-bindir" "old" "--new-bindir" "new" "--check" "--old-datadir" "old_data" "--new-datadir" "new_data" "--jobs" "10" "--link" "--no-sync" "--old-options" "old" "--new-options" "new" "--old-port" "5432" "--new-port" "5433" "--retain" "--socketdir" "socket" "--username" "user" "--verbose" "--version" "--clone" "--copy" "--help""#,
            command.to_command_string()
        );
    }
}
