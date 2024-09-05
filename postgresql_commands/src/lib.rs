#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![allow(async_fn_in_trait)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::struct_excessive_bools)]

//! Command builders for interacting with `PostgreSQL` via CLI.
//!
//! The commands are implemented as builders, which can be used to construct a
//! [standard Command](std::process::Command) or [tokio Command](tokio::process::Command).

pub mod clusterdb;
pub mod createdb;
pub mod createuser;
pub mod dropdb;
pub mod dropuser;
pub mod ecpg;
pub mod error;
pub mod initdb;
pub mod oid2name;
pub mod pg_amcheck;
pub mod pg_archivecleanup;
pub mod pg_basebackup;
pub mod pg_checksums;
pub mod pg_config;
pub mod pg_controldata;
pub mod pg_ctl;
pub mod pg_dump;
pub mod pg_dumpall;
pub mod pg_isready;
pub mod pg_receivewal;
pub mod pg_recvlogical;
pub mod pg_resetwal;
pub mod pg_restore;
pub mod pg_rewind;
pub mod pg_test_fsync;
pub mod pg_test_timing;
pub mod pg_upgrade;
pub mod pg_verifybackup;
pub mod pg_waldump;
pub mod pgbench;
pub mod postgres;
pub mod psql;
pub mod reindexdb;
pub mod traits;
pub mod vacuumdb;
pub mod vacuumlo;

pub use error::{Error, Result};
#[cfg(test)]
pub use traits::TestSettings;
pub use traits::{AsyncCommandExecutor, CommandBuilder, CommandExecutor, Settings};
