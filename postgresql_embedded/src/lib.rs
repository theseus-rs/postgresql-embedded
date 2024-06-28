//! # postgresql_embedded
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/postgresql-embedded/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/postgresql-embedded)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-postgresql-embedded)
//! [![License](https://img.shields.io/crates/l/postgresql_embedded)](https://github.com/theseus-rs/postgresql-embedded/tree/main/postgresql_embedded#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! Install and run a PostgreSQL database locally on Linux, MacOS or Windows.  PostgreSQL can be
//! bundled with your application, or downloaded on demand.
//!
//! ## Table of contents
//!
//! - [Examples](#examples)
//! - [Information](#information)
//! - [Feature flags](#feature-flags)
//! - [Safety](#safety)
//! - [License](#license)
//! - [Notes](#notes)
//!
//! ## Examples
//!
//! ### Asynchronous API
//!
//! ```no_run
//! use postgresql_embedded::{PostgreSQL, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let mut postgresql = PostgreSQL::default();
//!     postgresql.setup().await?;
//!     postgresql.start().await?;
//!
//!     let database_name = "test";
//!     postgresql.create_database(database_name).await?;
//!     postgresql.database_exists(database_name).await?;
//!     postgresql.drop_database(database_name).await?;
//!
//!     postgresql.stop().await
//! }
//! ```
//!
//! ### Synchronous API
//! ```no_run
//! #[cfg(feature = "blocking")] {
//! use postgresql_embedded::blocking::PostgreSQL;
//!
//! let mut postgresql = PostgreSQL::default();
//! postgresql.setup().unwrap();
//! postgresql.start().unwrap();
//!
//! let database_name = "test";
//! postgresql.create_database(database_name).unwrap();
//! postgresql.database_exists(database_name).unwrap();
//! postgresql.drop_database(database_name).unwrap();
//!
//! postgresql.stop().unwrap();
//! }
//! ```
//!
//! ## Information
//!
//! During the build process, when the `bundled` feature is enabled, the PostgreSQL binaries are
//! downloaded and included in the resulting binary. The version of the PostgreSQL binaries is
//! determined by the `POSTGRESQL_VERSION` environment variable. If the `POSTGRESQL_VERSION`
//! environment variable is not set, then `postgresql_archive::LATEST` will be used to determine the
//! version of the PostgreSQL binaries to download.
//!
//! When downloading the PostgreSQL binaries, either during build, or at runtime, the `GITHUB_TOKEN`
//! environment variable can be set to a GitHub personal access token to increase the rate limit for
//! downloading the PostgreSQL binaries. The `GITHUB_TOKEN` environment variable is not required.
//!
//! At runtime, the PostgreSQL binaries are cached by default in the following directories:
//!
//! - Unix: `$HOME/.theseus/postgresql`
//! - Windows: `%USERPROFILE%\.theseus\postgresql`
//!
//! ## Feature flags
//!
//! postgresql_embedded uses feature flags to address compile time and binary size
//! uses.
//!
//! The following features are available:
//!
//! | Name         | Description                                              | Default? |
//! |--------------|----------------------------------------------------------|----------|
//! | `bundled`    | Bundles the PostgreSQL archive into the resulting binary | No       |
//! | `blocking`   | Enables the blocking API; requires `tokio`               | No       |
//! | `native-tls` | Enables native-tls support                               | No       |
//! | `rustls-tls` | Enables rustls-tls support                               | Yes      |
//! | `tokio`      | Enables using tokio for async                            | No       |
//!
//! ## Safety
//!
//! These crates use `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.
//!
//! ## License
//!
//! Licensed under either of
//!
//! * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! PostgreSQL is covered under [The PostgreSQL License](https://opensource.org/licenses/postgresql).
//!
//! ## Notes
//!
//! Uses PostgreSQL binaries from [theseus-rs/postgresql-binaries](https://github.com/theseus-rs/postgresql-binaries).

#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![allow(dead_code)]
#![allow(clippy::doc_markdown)]
#![allow(deprecated)]

#[cfg(feature = "blocking")]
pub mod blocking;
mod error;
mod postgresql;
mod settings;

pub use error::{Error, Result};
pub use postgresql::{PostgreSQL, Status};
pub use postgresql_archive::{Version, VersionReq};
pub use settings::Settings;

lazy_static::lazy_static! {
    /// The latest PostgreSQL version requirement
    pub static ref LATEST: VersionReq = VersionReq::STAR;

    /// The latest PostgreSQL version 16
    pub static ref V16: VersionReq = VersionReq::parse("=16").unwrap();

    /// The latest PostgreSQL version 15
    pub static ref V15: VersionReq = VersionReq::parse("=15").unwrap();

    /// The latest PostgreSQL version 14
    pub static ref V14: VersionReq = VersionReq::parse("=14").unwrap();

    /// The latest PostgreSQL version 13
    pub static ref V13: VersionReq = VersionReq::parse("=13").unwrap();

    /// The latest PostgreSQL version 12
    #[deprecated(
        since = "0.1.0",
        note = "See https://www.postgresql.org/developer/roadmap/"
    )]
    pub static ref V12: VersionReq = VersionReq::parse("=12").unwrap();
}
