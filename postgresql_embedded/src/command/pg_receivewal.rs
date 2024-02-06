use crate::command::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// pg_receivewal receives PostgreSQL streaming write-ahead logs.
#[derive(Clone, Debug, Default)]
pub struct PgReceiveWalBuilder {
    program_dir: Option<PathBuf>,
    directory: Option<OsString>,
    endpos: Option<OsString>,
    if_not_exists: bool,
    no_loop: bool,
    no_sync: bool,
    status_interval: Option<OsString>,
    slot: Option<OsString>,
    synchronous: bool,
    verbose: bool,
    version: bool,
    compress: Option<OsString>,
    help: bool,
    dbname: Option<OsString>,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
    create_slot: bool,
    drop_slot: bool,
}

impl PgReceiveWalBuilder {
    /// Create a new [`PgReceiveWalBuilder`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Location of the program binary
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// receive write-ahead log files into this directory
    pub fn directory<S: AsRef<OsStr>>(mut self, directory: S) -> Self {
        self.directory = Some(directory.as_ref().to_os_string());
        self
    }

    /// exit after receiving the specified LSN
    pub fn endpos<S: AsRef<OsStr>>(mut self, endpos: S) -> Self {
        self.endpos = Some(endpos.as_ref().to_os_string());
        self
    }

    /// do not error if slot already exists when creating a slot
    pub fn if_not_exists(mut self) -> Self {
        self.if_not_exists = true;
        self
    }

    /// do not loop on connection lost
    pub fn no_loop(mut self) -> Self {
        self.no_loop = true;
        self
    }

    /// do not wait for changes to be written safely to disk
    pub fn no_sync(mut self) -> Self {
        self.no_sync = true;
        self
    }

    /// time between status packets sent to server (default: 10)
    pub fn status_interval<S: AsRef<OsStr>>(mut self, status_interval: S) -> Self {
        self.status_interval = Some(status_interval.as_ref().to_os_string());
        self
    }

    /// replication slot to use
    pub fn slot<S: AsRef<OsStr>>(mut self, slot: S) -> Self {
        self.slot = Some(slot.as_ref().to_os_string());
        self
    }

    /// flush write-ahead log immediately after writing
    pub fn synchronous(mut self) -> Self {
        self.synchronous = true;
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

    /// compress as specified
    pub fn compress<S: AsRef<OsStr>>(mut self, compress: S) -> Self {
        self.compress = Some(compress.as_ref().to_os_string());
        self
    }

    /// show help, then exit
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }

    /// connection string
    pub fn dbname<S: AsRef<OsStr>>(mut self, dbname: S) -> Self {
        self.dbname = Some(dbname.as_ref().to_os_string());
        self
    }

    /// database server host or socket directory
    pub fn host<S: AsRef<OsStr>>(mut self, host: S) -> Self {
        self.host = Some(host.as_ref().to_os_string());
        self
    }

    /// database server port number
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// connect as specified database user
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
        self
    }

    /// never prompt for password
    pub fn no_password(mut self) -> Self {
        self.no_password = true;
        self
    }

    /// force password prompt (should happen automatically)
    pub fn password(mut self) -> Self {
        self.password = true;
        self
    }

    /// create a new replication slot (for the slot's name see --slot)
    pub fn create_slot(mut self) -> Self {
        self.create_slot = true;
        self
    }

    /// drop the replication slot (for the slot's name see --slot)
    pub fn drop_slot(mut self) -> Self {
        self.drop_slot = true;
        self
    }
}

impl CommandBuilder for PgReceiveWalBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_receivewal".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(directory) = &self.directory {
            args.push("--directory".into());
            args.push(directory.into());
        }

        if let Some(endpos) = &self.endpos {
            args.push("--endpos".into());
            args.push(endpos.into());
        }

        if self.if_not_exists {
            args.push("--if-not-exists".into());
        }

        if self.no_loop {
            args.push("--no-loop".into());
        }

        if self.no_sync {
            args.push("--no-sync".into());
        }

        if let Some(status_interval) = &self.status_interval {
            args.push("--status-interval".into());
            args.push(status_interval.into());
        }

        if let Some(slot) = &self.slot {
            args.push("--slot".into());
            args.push(slot.into());
        }

        if self.synchronous {
            args.push("--synchronous".into());
        }

        if self.verbose {
            args.push("--verbose".into());
        }

        if self.version {
            args.push("--version".into());
        }

        if let Some(compress) = &self.compress {
            args.push("--compress".into());
            args.push(compress.into());
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

        if self.create_slot {
            args.push("--create-slot".into());
        }

        if self.drop_slot {
            args.push("--drop-slot".into());
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
        let command = PgReceiveWalBuilder::new().program_dir(".").build();

        assert_eq!(
            PathBuf::from(".").join("pg_receivewal"),
            PathBuf::from(command.to_command_string().replace("\"", ""))
        );
    }

    #[test]
    fn test_builder() {
        let command = PgReceiveWalBuilder::new()
            .directory("directory")
            .endpos("endpos")
            .if_not_exists()
            .no_loop()
            .no_sync()
            .status_interval("status_interval")
            .slot("slot")
            .synchronous()
            .verbose()
            .version()
            .compress("compress")
            .help()
            .dbname("dbname")
            .host("localhost")
            .port(5432)
            .username("username")
            .no_password()
            .password()
            .create_slot()
            .drop_slot()
            .build();

        assert_eq!(
            r#""pg_receivewal" "--directory" "directory" "--endpos" "endpos" "--if-not-exists" "--no-loop" "--no-sync" "--status-interval" "status_interval" "--slot" "slot" "--synchronous" "--verbose" "--version" "--compress" "compress" "--help" "--dbname" "dbname" "--host" "localhost" "--port" "5432" "--username" "username" "--no-password" "--password" "--create-slot" "--drop-slot""#,
            command.to_command_string()
        );
    }
}
