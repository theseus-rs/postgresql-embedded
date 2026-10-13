use crate::Settings;
use crate::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_upgrade` upgrades a `PostgreSQL` cluster to a different major version.
#[derive(Clone, Debug, Default)]
pub struct PgUpgradeBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    copy_file_range: bool,
    sync_method: Option<OsString>,
    no_statistics: bool,
    set_char_signedness: Option<OsString>,
    swap: bool,
    old_bindir: Option<OsString>,
    new_bindir: Option<OsString>,
    check: bool,
    old_datadir: Option<OsString>,
    new_datadir: Option<OsString>,
    jobs: Option<OsString>,
    link: bool,
    no_sync: bool,
    old_options: Vec<OsString>,
    new_options: Vec<OsString>,
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
    /// Create a new [`PgUpgradeBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgUpgradeBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new().program_dir(settings.get_binary_dir())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// old cluster executable directory
    #[must_use]
    pub fn old_bindir<S: AsRef<OsStr>>(mut self, old_bindir: S) -> Self {
        self.old_bindir = Some(old_bindir.as_ref().to_os_string());
        self
    }

    /// new cluster executable directory
    #[must_use]
    pub fn new_bindir<S: AsRef<OsStr>>(mut self, new_bindir: S) -> Self {
        self.new_bindir = Some(new_bindir.as_ref().to_os_string());
        self
    }

    /// check clusters only, don't change any data
    #[must_use]
    pub fn check(mut self) -> Self {
        self.check = true;
        self
    }

    /// old cluster data directory
    #[must_use]
    pub fn old_datadir<S: AsRef<OsStr>>(mut self, old_datadir: S) -> Self {
        self.old_datadir = Some(old_datadir.as_ref().to_os_string());
        self
    }

    /// new cluster data directory
    #[must_use]
    pub fn new_datadir<S: AsRef<OsStr>>(mut self, new_datadir: S) -> Self {
        self.new_datadir = Some(new_datadir.as_ref().to_os_string());
        self
    }

    /// number of simultaneous processes or threads to use
    #[must_use]
    pub fn jobs<S: AsRef<OsStr>>(mut self, jobs: S) -> Self {
        self.jobs = Some(jobs.as_ref().to_os_string());
        self
    }

    /// link instead of copying files to new cluster
    #[must_use]
    pub fn link(mut self) -> Self {
        self.link = true;
        self
    }

    /// do not wait for changes to be written safely to disk
    #[must_use]
    pub fn no_sync(mut self) -> Self {
        self.no_sync = true;
        self
    }

    /// old cluster options to pass to the server
    /// May be called multiple times; each value is retained.
    #[must_use]
    pub fn old_options<S: AsRef<OsStr>>(mut self, old_options: S) -> Self {
        self.old_options.push(old_options.as_ref().to_os_string());
        self
    }

    /// new cluster options to pass to the server
    /// May be called multiple times; each value is retained.
    #[must_use]
    pub fn new_options<S: AsRef<OsStr>>(mut self, new_options: S) -> Self {
        self.new_options.push(new_options.as_ref().to_os_string());
        self
    }

    /// old cluster port number
    #[must_use]
    pub fn old_port(mut self, old_port: u16) -> Self {
        self.old_port = Some(old_port);
        self
    }

    /// new cluster port number
    #[must_use]
    pub fn new_port(mut self, new_port: u16) -> Self {
        self.new_port = Some(new_port);
        self
    }

    /// retain SQL and log files after success
    #[must_use]
    pub fn retain(mut self) -> Self {
        self.retain = true;
        self
    }

    /// socket directory to use
    #[must_use]
    pub fn socketdir<S: AsRef<OsStr>>(mut self, socketdir: S) -> Self {
        self.socketdir = Some(socketdir.as_ref().to_os_string());
        self
    }

    /// cluster superuser
    #[must_use]
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
        self
    }

    /// enable verbose internal logging
    #[must_use]
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// display version information, then exit
    #[must_use]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// clone instead of copying files to new cluster
    #[must_use]
    pub fn clone(mut self) -> Self {
        self.clone = true;
        self
    }

    /// copy files to new cluster
    #[must_use]
    pub fn copy(mut self) -> Self {
        self.copy = true;
        self
    }

    /// show help, then exit
    #[must_use]
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }

    /// Copy files using the `copy_file_range` system call.
    #[must_use]
    pub fn copy_file_range(mut self) -> Self {
        self.copy_file_range = true;
        self
    }

    /// Select the file synchronization method.
    #[must_use]
    pub fn sync_method<S: AsRef<OsStr>>(mut self, sync_method: S) -> Self {
        self.sync_method = Some(sync_method.as_ref().to_os_string());
        self
    }

    /// Skip transfer of optimizer statistics.
    #[must_use]
    pub fn no_statistics(mut self) -> Self {
        self.no_statistics = true;
        self
    }

    /// Set the new cluster char signedness: signed or unsigned.
    #[must_use]
    pub fn set_char_signedness<S: AsRef<OsStr>>(mut self, set_char_signedness: S) -> Self {
        self.set_char_signedness = Some(set_char_signedness.as_ref().to_os_string());
        self
    }

    /// Move relation files to the new cluster.
    #[must_use]
    pub fn swap(mut self) -> Self {
        self.swap = true;
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

        if self.version {
            args.push("--version".into());
        }

        if self.help {
            args.push("--help".into());
        }

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

        for old_options in &self.old_options {
            args.push("--old-options".into());
            args.push(old_options.into());
        }

        for new_options in &self.new_options {
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

        if self.clone {
            args.push("--clone".into());
        }

        if self.copy {
            args.push("--copy".into());
        }

        if self.copy_file_range {
            args.push("--copy-file-range".into());
        }

        if let Some(sync_method) = &self.sync_method {
            args.push("--sync-method".into());
            args.push(sync_method.into());
        }

        if self.no_statistics {
            args.push("--no-statistics".into());
        }

        if let Some(set_char_signedness) = &self.set_char_signedness {
            args.push("--set-char-signedness".into());
            args.push(set_char_signedness.into());
        }

        if self.swap {
            args.push("--swap".into());
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
        let command = PgUpgradeBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_upgrade"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgUpgradeBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#""./pg_upgrade""#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\pg_upgrade""#;

        assert_eq!(format!("{command_prefix}"), command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = PgUpgradeBuilder::new()
            .env("PGDATABASE", "database")
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
            .copy_file_range()
            .sync_method("fsync")
            .no_statistics()
            .set_char_signedness("unsigned")
            .swap()
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"pg_upgrade" "--version" "--help" "--old-bindir" "old" "--new-bindir" "new" "--check" "--old-datadir" "old_data" "--new-datadir" "new_data" "--jobs" "10" "--link" "--no-sync" "--old-options" "old" "--new-options" "new" "--old-port" "5432" "--new-port" "5433" "--retain" "--socketdir" "socket" "--username" "user" "--verbose" "--clone" "--copy" "--copy-file-range" "--sync-method" "fsync" "--no-statistics" "--set-char-signedness" "unsigned" "--swap""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_options() {
        assert_args(
            PgUpgradeBuilder::new().copy_file_range(),
            &["--copy-file-range"],
        );
        assert_args(
            PgUpgradeBuilder::new().sync_method("fsync"),
            &["--sync-method", "fsync"],
        );
        assert_args(
            PgUpgradeBuilder::new().no_statistics(),
            &["--no-statistics"],
        );
        assert_args(
            PgUpgradeBuilder::new().set_char_signedness("unsigned"),
            &["--set-char-signedness", "unsigned"],
        );
        assert_args(PgUpgradeBuilder::new().swap(), &["--swap"]);
    }

    #[test]
    fn test_repeated_options() {
        assert_args(
            PgUpgradeBuilder::new()
                .old_options("first")
                .old_options("second"),
            &["--old-options", "first", "--old-options", "second"],
        );
        assert_args(
            PgUpgradeBuilder::new()
                .new_options("first")
                .new_options("second"),
            &["--new-options", "first", "--new-options", "second"],
        );
    }

    #[test]
    fn test_informational_options_come_first_on_configured_builders() {
        let version = PgUpgradeBuilder::from(&TestSettings).version().get_args();
        assert_eq!(version.first(), Some(&OsString::from("--version")));
        let help = PgUpgradeBuilder::from(&TestSettings).help().get_args();
        assert_eq!(help.first(), Some(&OsString::from("--help")));
    }

    #[test]
    fn test_cli_help_and_version() -> TestResult {
        let Some(bin) = binary_dir() else {
            return Ok(());
        };
        PgUpgradeBuilder::new()
            .program_dir(&bin)
            .version()
            .build()
            .execute()?;
        PgUpgradeBuilder::new()
            .program_dir(&bin)
            .help()
            .build()
            .execute()?;
        Ok(())
    }
}
