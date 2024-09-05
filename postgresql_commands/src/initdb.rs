use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `initdb` initializes a `PostgreSQL` database cluster.
#[derive(Clone, Debug, Default)]
pub struct InitDbBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    auth: Option<OsString>,
    auth_host: Option<OsString>,
    auth_local: Option<OsString>,
    pgdata: Option<PathBuf>,
    encoding: Option<OsString>,
    allow_group_access: bool,
    icu_locale: Option<OsString>,
    icu_rules: Option<OsString>,
    data_checksums: bool,
    locale: Option<OsString>,
    lc_collate: Option<OsString>,
    lc_ctype: Option<OsString>,
    lc_messages: Option<OsString>,
    lc_monetary: Option<OsString>,
    lc_numeric: Option<OsString>,
    lc_time: Option<OsString>,
    no_locale: bool,
    locale_provider: Option<OsString>,
    pwfile: Option<PathBuf>,
    text_search_config: Option<OsString>,
    username: Option<OsString>,
    pwprompt: bool,
    waldir: Option<OsString>,
    wal_segsize: Option<OsString>,
    set: Option<OsString>,
    debug: bool,
    discard_caches: bool,
    directory: Option<OsString>,
    no_clean: bool,
    no_sync: bool,
    no_instructions: bool,
    show: bool,
    sync_only: bool,
    version: bool,
    help: bool,
}

impl InitDbBuilder {
    /// Create a new [`InitDbBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`InitDbBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new()
            .program_dir(settings.get_binary_dir())
            .username(settings.get_username())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// Set the default authentication method for local connections
    #[must_use]
    pub fn auth<S: AsRef<OsStr>>(mut self, auth: S) -> Self {
        self.auth = Some(auth.as_ref().to_os_string());
        self
    }

    /// Set the default authentication method for local TCP/IP connections
    #[must_use]
    pub fn auth_host<S: AsRef<OsStr>>(mut self, auth_host: S) -> Self {
        self.auth_host = Some(auth_host.as_ref().to_os_string());
        self
    }

    /// Set the default authentication method for local-socket connections
    #[must_use]
    pub fn auth_local<S: AsRef<OsStr>>(mut self, auth_local: S) -> Self {
        self.auth_local = Some(auth_local.as_ref().to_os_string());
        self
    }

    /// Set the location for this database cluster
    #[must_use]
    pub fn pgdata<P: Into<PathBuf>>(mut self, pgdata: P) -> Self {
        self.pgdata = Some(pgdata.into());
        self
    }

    /// Set the default encoding for new databases
    #[must_use]
    pub fn encoding<S: AsRef<OsStr>>(mut self, encoding: S) -> Self {
        self.encoding = Some(encoding.as_ref().to_os_string());
        self
    }

    /// Allow group read/execute on data directory
    #[must_use]
    pub fn allow_group_access(mut self) -> Self {
        self.allow_group_access = true;
        self
    }

    /// Set the ICU locale ID for new databases
    #[must_use]
    pub fn icu_locale<S: AsRef<OsStr>>(mut self, icu_locale: S) -> Self {
        self.icu_locale = Some(icu_locale.as_ref().to_os_string());
        self
    }

    /// Set additional ICU collation rules for new databases
    #[must_use]
    pub fn icu_rules<S: AsRef<OsStr>>(mut self, icu_rules: S) -> Self {
        self.icu_rules = Some(icu_rules.as_ref().to_os_string());
        self
    }

    /// Use data page checksums
    #[must_use]
    pub fn data_checksums(mut self) -> Self {
        self.data_checksums = true;
        self
    }

    /// Set the default locale for new databases
    #[must_use]
    pub fn locale<S: AsRef<OsStr>>(mut self, locale: S) -> Self {
        self.locale = Some(locale.as_ref().to_os_string());
        self
    }

    /// Set the default locale in the respective category for new databases
    #[must_use]
    pub fn lc_collate<S: AsRef<OsStr>>(mut self, lc_collate: S) -> Self {
        self.lc_collate = Some(lc_collate.as_ref().to_os_string());
        self
    }

    /// Set the default locale in the respective category for new databases
    #[must_use]
    pub fn lc_ctype<S: AsRef<OsStr>>(mut self, lc_ctype: S) -> Self {
        self.lc_ctype = Some(lc_ctype.as_ref().to_os_string());
        self
    }

    /// Set the default locale in the respective category for new databases
    #[must_use]
    pub fn lc_messages<S: AsRef<OsStr>>(mut self, lc_messages: S) -> Self {
        self.lc_messages = Some(lc_messages.as_ref().to_os_string());
        self
    }

    /// Set the default locale in the respective category for new databases
    #[must_use]
    pub fn lc_monetary<S: AsRef<OsStr>>(mut self, lc_monetary: S) -> Self {
        self.lc_monetary = Some(lc_monetary.as_ref().to_os_string());
        self
    }

    /// Set the default locale in the respective category for new databases
    #[must_use]
    pub fn lc_numeric<S: AsRef<OsStr>>(mut self, lc_numeric: S) -> Self {
        self.lc_numeric = Some(lc_numeric.as_ref().to_os_string());
        self
    }

    /// Set the default locale in the respective category for new databases
    #[must_use]
    pub fn lc_time<S: AsRef<OsStr>>(mut self, lc_time: S) -> Self {
        self.lc_time = Some(lc_time.as_ref().to_os_string());
        self
    }

    /// Equivalent to --locale=C
    #[must_use]
    pub fn no_locale(mut self) -> Self {
        self.no_locale = true;
        self
    }

    /// Set the default locale provider for new databases
    #[must_use]
    pub fn locale_provider<S: AsRef<OsStr>>(mut self, locale_provider: S) -> Self {
        self.locale_provider = Some(locale_provider.as_ref().to_os_string());
        self
    }

    /// Read password for the new superuser from file
    #[must_use]
    pub fn pwfile<P: Into<PathBuf>>(mut self, pwfile: P) -> Self {
        self.pwfile = Some(pwfile.into());
        self
    }

    /// Set the default text search configuration
    #[must_use]
    pub fn text_search_config<S: AsRef<OsStr>>(mut self, text_search_config: S) -> Self {
        self.text_search_config = Some(text_search_config.as_ref().to_os_string());
        self
    }

    /// Set the database superuser name
    #[must_use]
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
        self
    }

    /// Prompt for a password for the new superuser
    #[must_use]
    pub fn pwprompt(mut self) -> Self {
        self.pwprompt = true;
        self
    }

    /// Set the location for the write-ahead log directory
    #[must_use]
    pub fn waldir<S: AsRef<OsStr>>(mut self, waldir: S) -> Self {
        self.waldir = Some(waldir.as_ref().to_os_string());
        self
    }

    /// Set the size of WAL segments, in megabytes
    #[must_use]
    pub fn wal_segsize<S: AsRef<OsStr>>(mut self, wal_segsize: S) -> Self {
        self.wal_segsize = Some(wal_segsize.as_ref().to_os_string());
        self
    }

    /// Override default setting for server parameter
    #[must_use]
    pub fn set<S: AsRef<OsStr>>(mut self, set: S) -> Self {
        self.set = Some(set.as_ref().to_os_string());
        self
    }

    /// Generate lots of debugging output
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    /// Set `debug_discard_caches=1`
    #[must_use]
    pub fn discard_caches(mut self) -> Self {
        self.discard_caches = true;
        self
    }

    /// Set where to find the input files
    #[must_use]
    pub fn directory<S: AsRef<OsStr>>(mut self, directory: S) -> Self {
        self.directory = Some(directory.as_ref().to_os_string());
        self
    }

    /// Do not clean up after errors
    #[must_use]
    pub fn no_clean(mut self) -> Self {
        self.no_clean = true;
        self
    }

    /// Do not wait for changes to be written safely to disk
    #[must_use]
    pub fn no_sync(mut self) -> Self {
        self.no_sync = true;
        self
    }

    /// Do not print instructions for next steps
    #[must_use]
    pub fn no_instructions(mut self) -> Self {
        self.no_instructions = true;
        self
    }

    /// Show internal settings
    #[must_use]
    pub fn show(mut self) -> Self {
        self.show = true;
        self
    }

    /// Only sync database files to disk, then exit
    #[must_use]
    pub fn sync_only(mut self) -> Self {
        self.sync_only = true;
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

impl CommandBuilder for InitDbBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "initdb".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    #[expect(clippy::too_many_lines)]
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(auth) = &self.auth {
            args.push("--auth".into());
            args.push(auth.into());
        }

        if let Some(auth_host) = &self.auth_host {
            args.push("--auth-host".into());
            args.push(auth_host.into());
        }

        if let Some(auth_local) = &self.auth_local {
            args.push("--auth-local".into());
            args.push(auth_local.into());
        }

        if let Some(pgdata) = &self.pgdata {
            args.push("--pgdata".into());
            args.push(pgdata.into());
        }

        if let Some(encoding) = &self.encoding {
            args.push("--encoding".into());
            args.push(encoding.into());
        }

        if self.allow_group_access {
            args.push("--allow-group-access".into());
        }

        if let Some(icu_locale) = &self.icu_locale {
            args.push("--icu-locale".into());
            args.push(icu_locale.into());
        }

        if let Some(icu_rules) = &self.icu_rules {
            args.push("--icu-rules".into());
            args.push(icu_rules.into());
        }

        if self.data_checksums {
            args.push("--data-checksums".into());
        }

        if let Some(locale) = &self.locale {
            args.push("--locale".into());
            args.push(locale.into());
        }

        if let Some(lc_collate) = &self.lc_collate {
            args.push("--lc-collate".into());
            args.push(lc_collate.into());
        }

        if let Some(lc_ctype) = &self.lc_ctype {
            args.push("--lc-ctype".into());
            args.push(lc_ctype.into());
        }

        if let Some(lc_messages) = &self.lc_messages {
            args.push("--lc-messages".into());
            args.push(lc_messages.into());
        }

        if let Some(lc_monetary) = &self.lc_monetary {
            args.push("--lc-monetary".into());
            args.push(lc_monetary.into());
        }

        if let Some(lc_numeric) = &self.lc_numeric {
            args.push("--lc-numeric".into());
            args.push(lc_numeric.into());
        }

        if let Some(lc_time) = &self.lc_time {
            args.push("--lc-time".into());
            args.push(lc_time.into());
        }

        if self.no_locale {
            args.push("--no-locale".into());
        }

        if let Some(locale_provider) = &self.locale_provider {
            args.push("--locale-provider".into());
            args.push(locale_provider.into());
        }

        if let Some(pwfile) = &self.pwfile {
            args.push("--pwfile".into());
            args.push(pwfile.into());
        }

        if let Some(text_search_config) = &self.text_search_config {
            args.push("--text-search-config".into());
            args.push(text_search_config.into());
        }

        if let Some(username) = &self.username {
            args.push("--username".into());
            args.push(username.into());
        }

        if self.pwprompt {
            args.push("--pwprompt".into());
        }

        if let Some(waldir) = &self.waldir {
            args.push("--waldir".into());
            args.push(waldir.into());
        }

        if let Some(wal_segsize) = &self.wal_segsize {
            args.push("--wal-segsize".into());
            args.push(wal_segsize.into());
        }

        if let Some(set) = &self.set {
            args.push("--set".into());
            args.push(set.into());
        }

        if self.debug {
            args.push("--debug".into());
        }

        if self.discard_caches {
            args.push("--discard-caches".into());
        }

        if let Some(directory) = &self.directory {
            args.push("--directory".into());
            args.push(directory.into());
        }

        if self.no_clean {
            args.push("--no-clean".into());
        }

        if self.no_sync {
            args.push("--no-sync".into());
        }

        if self.no_instructions {
            args.push("--no-instructions".into());
        }

        if self.show {
            args.push("--show".into());
        }

        if self.sync_only {
            args.push("--sync-only".into());
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
        let command = InitDbBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("initdb"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = InitDbBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#""./initdb" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\initdb" "#;

        assert_eq!(
            format!(r#"{command_prefix}"--username" "postgres""#),
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder() {
        let command = InitDbBuilder::new()
            .env("PGDATABASE", "database")
            .auth("md5")
            .auth_host("md5")
            .auth_local("md5")
            .pgdata("pgdata")
            .encoding("UTF8")
            .allow_group_access()
            .icu_locale("en_US")
            .icu_rules("phonebook")
            .data_checksums()
            .locale("en_US")
            .lc_collate("en_US")
            .lc_ctype("en_US")
            .lc_messages("en_US")
            .lc_monetary("en_US")
            .lc_numeric("en_US")
            .lc_time("en_US")
            .no_locale()
            .locale_provider("icu")
            .pwfile(".pwfile")
            .text_search_config("english")
            .username("postgres")
            .pwprompt()
            .waldir("waldir")
            .wal_segsize("1")
            .set("timezone=UTC")
            .debug()
            .discard_caches()
            .directory("directory")
            .no_clean()
            .no_sync()
            .no_instructions()
            .show()
            .sync_only()
            .version()
            .help()
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"initdb" "--auth" "md5" "--auth-host" "md5" "--auth-local" "md5" "--pgdata" "pgdata" "--encoding" "UTF8" "--allow-group-access" "--icu-locale" "en_US" "--icu-rules" "phonebook" "--data-checksums" "--locale" "en_US" "--lc-collate" "en_US" "--lc-ctype" "en_US" "--lc-messages" "en_US" "--lc-monetary" "en_US" "--lc-numeric" "en_US" "--lc-time" "en_US" "--no-locale" "--locale-provider" "icu" "--pwfile" ".pwfile" "--text-search-config" "english" "--username" "postgres" "--pwprompt" "--waldir" "waldir" "--wal-segsize" "1" "--set" "timezone=UTC" "--debug" "--discard-caches" "--directory" "directory" "--no-clean" "--no-sync" "--no-instructions" "--show" "--sync-only" "--version" "--help""#
            ),
            command.to_command_string()
        );
    }
}
