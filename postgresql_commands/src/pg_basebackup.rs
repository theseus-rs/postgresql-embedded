use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_basebackup` takes a base backup of a running `PostgreSQL` server.
#[derive(Clone, Debug, Default)]
pub struct PgBaseBackupBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    pgdata: Option<PathBuf>,
    format: Option<OsString>,
    max_rate: Option<OsString>,
    write_recovery_conf: bool,
    target: Option<OsString>,
    tablespace_mapping: Option<OsString>,
    waldir: Option<OsString>,
    wal_method: Option<OsString>,
    gzip: bool,
    compress: Option<OsString>,
    checkpoint: Option<OsString>,
    create_slot: bool,
    label: Option<OsString>,
    no_clean: bool,
    no_sync: bool,
    progress: bool,
    slot: Option<OsString>,
    verbose: bool,
    version: bool,
    manifest_checksums: Option<OsString>,
    manifest_force_encode: bool,
    no_estimate_size: bool,
    no_manifest: bool,
    no_slot: bool,
    no_verify_checksums: bool,
    help: bool,
    dbname: Option<OsString>,
    host: Option<OsString>,
    port: Option<u16>,
    status_interval: Option<OsString>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
    pg_password: Option<OsString>,
}

impl PgBaseBackupBuilder {
    /// Create a new [`PgBaseBackupBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgBaseBackupBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new()
            .program_dir(settings.get_binary_dir())
            .host(settings.get_host())
            .port(settings.get_port())
            .username(settings.get_username())
            .pg_password(settings.get_password())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// receive base backup into directory
    #[must_use]
    pub fn pgdata<P: Into<PathBuf>>(mut self, pgdata: P) -> Self {
        self.pgdata = Some(pgdata.into());
        self
    }

    /// output format (plain (default), tar)
    #[must_use]
    pub fn format<S: AsRef<OsStr>>(mut self, format: S) -> Self {
        self.format = Some(format.as_ref().to_os_string());
        self
    }

    /// maximum transfer rate to transfer data directory (in kB/s, or use suffix "k" or "M")
    #[must_use]
    pub fn max_rate<S: AsRef<OsStr>>(mut self, max_rate: S) -> Self {
        self.max_rate = Some(max_rate.as_ref().to_os_string());
        self
    }

    /// write configuration for replication
    #[must_use]
    pub fn write_recovery_conf(mut self) -> Self {
        self.write_recovery_conf = true;
        self
    }

    /// backup target (if other than client)
    #[must_use]
    pub fn target<S: AsRef<OsStr>>(mut self, target: S) -> Self {
        self.target = Some(target.as_ref().to_os_string());
        self
    }

    /// relocate tablespace in OLDDIR to NEWDIR
    #[must_use]
    pub fn tablespace_mapping<S: AsRef<OsStr>>(mut self, tablespace_mapping: S) -> Self {
        self.tablespace_mapping = Some(tablespace_mapping.as_ref().to_os_string());
        self
    }

    /// location for the write-ahead log directory
    #[must_use]
    pub fn waldir<S: AsRef<OsStr>>(mut self, waldir: S) -> Self {
        self.waldir = Some(waldir.as_ref().to_os_string());
        self
    }

    /// include required WAL files with specified method
    #[must_use]
    pub fn wal_method<S: AsRef<OsStr>>(mut self, wal_method: S) -> Self {
        self.wal_method = Some(wal_method.as_ref().to_os_string());
        self
    }

    /// compress tar output
    #[must_use]
    pub fn gzip(mut self) -> Self {
        self.gzip = true;
        self
    }

    /// compress on client or server as specified
    #[must_use]
    pub fn compress<S: AsRef<OsStr>>(mut self, compress: S) -> Self {
        self.compress = Some(compress.as_ref().to_os_string());
        self
    }

    /// set fast or spread checkpointing
    #[must_use]
    pub fn checkpoint<S: AsRef<OsStr>>(mut self, checkpoint: S) -> Self {
        self.checkpoint = Some(checkpoint.as_ref().to_os_string());
        self
    }

    /// create replication slot
    #[must_use]
    pub fn create_slot(mut self) -> Self {
        self.create_slot = true;
        self
    }

    /// set backup label
    #[must_use]
    pub fn label<S: AsRef<OsStr>>(mut self, label: S) -> Self {
        self.label = Some(label.as_ref().to_os_string());
        self
    }

    /// do not clean up after errors
    #[must_use]
    pub fn no_clean(mut self) -> Self {
        self.no_clean = true;
        self
    }

    /// do not wait for changes to be written safely to disk
    #[must_use]
    pub fn no_sync(mut self) -> Self {
        self.no_sync = true;
        self
    }

    /// show progress information
    #[must_use]
    pub fn progress(mut self) -> Self {
        self.progress = true;
        self
    }

    /// replication slot to use
    #[must_use]
    pub fn slot<S: AsRef<OsStr>>(mut self, slot: S) -> Self {
        self.slot = Some(slot.as_ref().to_os_string());
        self
    }

    /// output verbose messages
    #[must_use]
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// output version information, then exit
    #[must_use]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// use algorithm for manifest checksums
    #[must_use]
    pub fn manifest_checksums<S: AsRef<OsStr>>(mut self, manifest_checksums: S) -> Self {
        self.manifest_checksums = Some(manifest_checksums.as_ref().to_os_string());
        self
    }

    /// hex encode all file names in manifest
    #[must_use]
    pub fn manifest_force_encode(mut self) -> Self {
        self.manifest_force_encode = true;
        self
    }

    /// do not estimate backup size in server side
    #[must_use]
    pub fn no_estimate_size(mut self) -> Self {
        self.no_estimate_size = true;
        self
    }

    /// suppress generation of backup manifest
    #[must_use]
    pub fn no_manifest(mut self) -> Self {
        self.no_manifest = true;
        self
    }

    /// prevent creation of temporary replication slot
    #[must_use]
    pub fn no_slot(mut self) -> Self {
        self.no_slot = true;
        self
    }

    /// do not verify checksums
    #[must_use]
    pub fn no_verify_checksums(mut self) -> Self {
        self.no_verify_checksums = true;
        self
    }

    /// show this help, then exit
    #[must_use]
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }

    /// connection string
    #[must_use]
    pub fn dbname<S: AsRef<OsStr>>(mut self, dbname: S) -> Self {
        self.dbname = Some(dbname.as_ref().to_os_string());
        self
    }

    /// database server host or socket directory
    #[must_use]
    pub fn host<S: AsRef<OsStr>>(mut self, host: S) -> Self {
        self.host = Some(host.as_ref().to_os_string());
        self
    }

    /// database server port number
    #[must_use]
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// time between status packets sent to server (in seconds)
    #[must_use]
    pub fn status_interval<S: AsRef<OsStr>>(mut self, status_interval: S) -> Self {
        self.status_interval = Some(status_interval.as_ref().to_os_string());
        self
    }

    /// connect as specified database user
    #[must_use]
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
        self
    }

    /// never prompt for password
    #[must_use]
    pub fn no_password(mut self) -> Self {
        self.no_password = true;
        self
    }

    /// force password prompt (should happen automatically)
    #[must_use]
    pub fn password(mut self) -> Self {
        self.password = true;
        self
    }

    /// user password
    #[must_use]
    pub fn pg_password<S: AsRef<OsStr>>(mut self, pg_password: S) -> Self {
        self.pg_password = Some(pg_password.as_ref().to_os_string());
        self
    }
}

impl CommandBuilder for PgBaseBackupBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_basebackup".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    #[expect(clippy::too_many_lines)]
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(pgdata) = &self.pgdata {
            args.push("--pgdata".into());
            args.push(pgdata.into());
        }

        if let Some(format) = &self.format {
            args.push("--format".into());
            args.push(format.into());
        }

        if let Some(max_rate) = &self.max_rate {
            args.push("--max-rate".into());
            args.push(max_rate.into());
        }

        if self.write_recovery_conf {
            args.push("--write-recovery-conf".into());
        }

        if let Some(target) = &self.target {
            args.push("--target".into());
            args.push(target.into());
        }

        if let Some(tablespace_mapping) = &self.tablespace_mapping {
            args.push("--tablespace-mapping".into());
            args.push(tablespace_mapping.into());
        }

        if let Some(waldir) = &self.waldir {
            args.push("--waldir".into());
            args.push(waldir.into());
        }

        if let Some(wal_method) = &self.wal_method {
            args.push("--wal-method".into());
            args.push(wal_method.into());
        }

        if self.gzip {
            args.push("--gzip".into());
        }

        if let Some(compress) = &self.compress {
            args.push("--compress".into());
            args.push(compress.into());
        }

        if let Some(checkpoint) = &self.checkpoint {
            args.push("--checkpoint".into());
            args.push(checkpoint.into());
        }

        if self.create_slot {
            args.push("--create-slot".into());
        }

        if let Some(label) = &self.label {
            args.push("--label".into());
            args.push(label.into());
        }

        if self.no_clean {
            args.push("--no-clean".into());
        }

        if self.no_sync {
            args.push("--no-sync".into());
        }

        if self.progress {
            args.push("--progress".into());
        }

        if let Some(slot) = &self.slot {
            args.push("--slot".into());
            args.push(slot.into());
        }

        if self.verbose {
            args.push("--verbose".into());
        }

        if self.version {
            args.push("--version".into());
        }

        if let Some(manifest_checksums) = &self.manifest_checksums {
            args.push("--manifest-checksums".into());
            args.push(manifest_checksums.into());
        }

        if self.manifest_force_encode {
            args.push("--manifest-force-encode".into());
        }

        if self.no_estimate_size {
            args.push("--no-estimate-size".into());
        }

        if self.no_manifest {
            args.push("--no-manifest".into());
        }

        if self.no_slot {
            args.push("--no-slot".into());
        }

        if self.no_verify_checksums {
            args.push("--no-verify-checksums".into());
        }

        if self.help {
            args.push("--help".into());
        }

        if let Some(dbname) = &self.dbname {
            args.push("--dbname".into());
            args.push(dbname.into());
        }

        if let Some(host) = &self.host {
            args.push("--host".into());
            args.push(host.into());
        }

        if let Some(port) = &self.port {
            args.push("--port".into());
            args.push(port.to_string().into());
        }

        if let Some(status_interval) = &self.status_interval {
            args.push("--status-interval".into());
            args.push(status_interval.into());
        }

        if let Some(username) = &self.username {
            args.push("--username".into());
            args.push(username.into());
        }

        if self.no_password {
            args.push("--no-password".into());
        }

        if self.password {
            args.push("--password".into());
        }

        args
    }

    /// Get the environment variables for the command
    fn get_envs(&self) -> Vec<(OsString, OsString)> {
        let mut envs: Vec<(OsString, OsString)> = self.envs.clone();

        if let Some(password) = &self.pg_password {
            envs.push(("PGPASSWORD".into(), password.into()));
        }

        envs
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
        let command = PgBaseBackupBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_basebackup"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgBaseBackupBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGPASSWORD="password" "./pg_basebackup" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\pg_basebackup" "#;

        assert_eq!(
            format!(
                r#"{command_prefix}"--host" "localhost" "--port" "5432" "--username" "postgres""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder() {
        let command = PgBaseBackupBuilder::new()
            .env("PGDATABASE", "database")
            .pgdata("pgdata")
            .format("plain")
            .max_rate("100M")
            .write_recovery_conf()
            .target("localhost")
            .tablespace_mapping("tablespace_mapping")
            .waldir("waldir")
            .wal_method("stream")
            .gzip()
            .compress("client")
            .checkpoint("fast")
            .create_slot()
            .label("my_backup")
            .no_clean()
            .no_sync()
            .progress()
            .slot("my_slot")
            .verbose()
            .version()
            .manifest_checksums("sha256")
            .manifest_force_encode()
            .no_estimate_size()
            .no_manifest()
            .no_slot()
            .no_verify_checksums()
            .help()
            .dbname("postgres")
            .host("localhost")
            .port(5432)
            .status_interval("10")
            .username("postgres")
            .no_password()
            .password()
            .pg_password("password")
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" PGPASSWORD="password" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"pg_basebackup" "--pgdata" "pgdata" "--format" "plain" "--max-rate" "100M" "--write-recovery-conf" "--target" "localhost" "--tablespace-mapping" "tablespace_mapping" "--waldir" "waldir" "--wal-method" "stream" "--gzip" "--compress" "client" "--checkpoint" "fast" "--create-slot" "--label" "my_backup" "--no-clean" "--no-sync" "--progress" "--slot" "my_slot" "--verbose" "--version" "--manifest-checksums" "sha256" "--manifest-force-encode" "--no-estimate-size" "--no-manifest" "--no-slot" "--no-verify-checksums" "--help" "--dbname" "postgres" "--host" "localhost" "--port" "5432" "--status-interval" "10" "--username" "postgres" "--no-password" "--password""#
            ),
            command.to_command_string()
        );
    }
}
