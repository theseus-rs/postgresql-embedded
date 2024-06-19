use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_resetwal` resets the `PostgreSQL` write-ahead log.
#[derive(Clone, Debug, Default)]
pub struct PgResetWalBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    commit_timestamp_ids: Option<(OsString, OsString)>,
    pgdata: Option<PathBuf>,
    epoch: Option<OsString>,
    force: bool,
    next_wal_file: Option<OsString>,
    multixact_ids: Option<(OsString, OsString)>,
    dry_run: bool,
    next_oid: Option<OsString>,
    multixact_offset: Option<OsString>,
    oldest_transaction_id: Option<OsString>,
    version: bool,
    next_transaction_id: Option<OsString>,
    wal_segsize: Option<OsString>,
    help: bool,
}

impl PgResetWalBuilder {
    /// Create a new [`PgResetWalBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgResetWalBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new().program_dir(settings.get_binary_dir())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// set oldest and newest transactions bearing commit timestamp (zero means no change)
    #[must_use]
    pub fn commit_timestamp_ids<S: AsRef<OsStr>>(mut self, xid1: S, xid2: S) -> Self {
        self.commit_timestamp_ids = Some((xid1.as_ref().into(), xid2.as_ref().into()));
        self
    }

    /// data directory
    #[must_use]
    pub fn pgdata<P: Into<PathBuf>>(mut self, datadir: P) -> Self {
        self.pgdata = Some(datadir.into());
        self
    }

    /// set next transaction ID epoch
    #[must_use]
    pub fn epoch<S: AsRef<OsStr>>(mut self, xidepoch: S) -> Self {
        self.epoch = Some(xidepoch.as_ref().to_os_string());
        self
    }

    /// force update to be done
    #[must_use]
    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }

    /// set minimum starting location for new WAL
    #[must_use]
    pub fn next_wal_file<S: AsRef<OsStr>>(mut self, walfile: S) -> Self {
        self.next_wal_file = Some(walfile.as_ref().to_os_string());
        self
    }

    /// set next and oldest multitransaction ID
    #[must_use]
    pub fn multixact_ids<S: AsRef<OsStr>>(mut self, mxid1: S, mxid2: S) -> Self {
        self.multixact_ids = Some((mxid1.as_ref().into(), mxid2.as_ref().into()));
        self
    }

    /// no update, just show what would be done
    #[must_use]
    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }

    /// set next OID
    #[must_use]
    pub fn next_oid<S: AsRef<OsStr>>(mut self, oid: S) -> Self {
        self.next_oid = Some(oid.as_ref().to_os_string());
        self
    }

    /// set next multitransaction offset
    #[must_use]
    pub fn multixact_offset<S: AsRef<OsStr>>(mut self, offset: S) -> Self {
        self.multixact_offset = Some(offset.as_ref().to_os_string());
        self
    }

    /// set oldest transaction ID
    #[must_use]
    pub fn oldest_transaction_id<S: AsRef<OsStr>>(mut self, xid: S) -> Self {
        self.oldest_transaction_id = Some(xid.as_ref().to_os_string());
        self
    }

    /// output version information, then exit
    #[must_use]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// set next transaction ID
    #[must_use]
    pub fn next_transaction_id<S: AsRef<OsStr>>(mut self, xid: S) -> Self {
        self.next_transaction_id = Some(xid.as_ref().to_os_string());
        self
    }

    /// size of WAL segments, in megabytes
    #[must_use]
    pub fn wal_segsize<S: AsRef<OsStr>>(mut self, size: S) -> Self {
        self.wal_segsize = Some(size.as_ref().to_os_string());
        self
    }

    /// show help, then exit
    #[must_use]
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }
}

impl CommandBuilder for PgResetWalBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_resetwal".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some((xid1, xid2)) = &self.commit_timestamp_ids {
            args.push("--commit-timestamp-ids".into());
            args.push(format!("{},{}", xid1.to_string_lossy(), xid2.to_string_lossy()).into());
        }

        if let Some(datadir) = &self.pgdata {
            args.push("--pgdata".into());
            args.push(datadir.into());
        }

        if let Some(xidepoch) = &self.epoch {
            args.push("--epoch".into());
            args.push(xidepoch.into());
        }

        if self.force {
            args.push("--force".into());
        }

        if let Some(walfile) = &self.next_wal_file {
            args.push("--next-wal-file".into());
            args.push(walfile.into());
        }

        if let Some((mxid1, mxid2)) = &self.multixact_ids {
            args.push("--multixact-ids".into());
            args.push(format!("{},{}", mxid1.to_string_lossy(), mxid2.to_string_lossy()).into());
        }

        if self.dry_run {
            args.push("--dry-run".into());
        }

        if let Some(oid) = &self.next_oid {
            args.push("--next-oid".into());
            args.push(oid.into());
        }

        if let Some(offset) = &self.multixact_offset {
            args.push("--multixact-offset".into());
            args.push(offset.into());
        }

        if let Some(xid) = &self.oldest_transaction_id {
            args.push("--oldest-transaction-id".into());
            args.push(xid.into());
        }

        if self.version {
            args.push("--version".into());
        }

        if let Some(xid) = &self.next_transaction_id {
            args.push("--next-transaction-id".into());
            args.push(xid.into());
        }

        if let Some(size) = &self.wal_segsize {
            args.push("--wal-segsize".into());
            args.push(size.into());
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
        let command = PgResetWalBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_resetwal"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgResetWalBuilder::from(&TestSettings).build();
        assert_eq!(r#""./pg_resetwal""#, command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = PgResetWalBuilder::new()
            .env("PGDATABASE", "database")
            .commit_timestamp_ids("1", "2")
            .pgdata("pgdata")
            .epoch("epoch")
            .force()
            .next_wal_file("next_wal_file")
            .multixact_ids("3", "4")
            .dry_run()
            .next_oid("next_oid")
            .multixact_offset("multixact_offset")
            .oldest_transaction_id("oldest_transaction_id")
            .version()
            .next_transaction_id("next_transaction_id")
            .wal_segsize("wal_segsize")
            .help()
            .build();

        assert_eq!(
            r#"PGDATABASE="database" "pg_resetwal" "--commit-timestamp-ids" "1,2" "--pgdata" "pgdata" "--epoch" "epoch" "--force" "--next-wal-file" "next_wal_file" "--multixact-ids" "3,4" "--dry-run" "--next-oid" "next_oid" "--multixact-offset" "multixact_offset" "--oldest-transaction-id" "oldest_transaction_id" "--version" "--next-transaction-id" "next_transaction_id" "--wal-segsize" "wal_segsize" "--help""#,
            command.to_command_string()
        );
    }
}
