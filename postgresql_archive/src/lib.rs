//! # postgresql_archive
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/postgresql-embedded/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/postgresql-embedded)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-postgresql-embedded)
//! [![License](https://img.shields.io/crates/l/postgresql_archive?)](https://github.com/theseus-rs/postgresql-embedded/tree/main/postgresql_archive#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! Retrieve and extract PostgreSQL on Linux, MacOS or Windows.
//!
//! ## Table of contents
//!
//! - [Examples](#examples)
//! - [Feature flags](#feature-flags)
//! - [Supported platforms](#supported-platforms)
//! - [Safety](#safety)
//! - [License](#license)
//! - [Notes](#notes)
//!
//! ## Examples
//!
//! ### Asynchronous API
//!
//! ```no_run
//! use postgresql_archive::{extract, get_archive, Result, VersionReq };
//! use postgresql_archive::configuration::theseus;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let url = theseus::URL;
//!     let (archive_version, archive) = get_archive(url, &VersionReq::STAR).await?;
//!     let out_dir = std::env::temp_dir();
//!     let files = extract(url, &archive, &out_dir).await?;
//!     Ok(())
//! }
//! ```
//!
//! ### Synchronous API
//! ```no_run
//! #[cfg(feature = "blocking")] {
//! use postgresql_archive::configuration::theseus;
//! use postgresql_archive::VersionReq;
//! use postgresql_archive::blocking::{extract, get_archive};
//!
//! let url = theseus::URL;
//! let (archive_version, archive) = get_archive(url, &VersionReq::STAR).unwrap();
//! let out_dir = std::env::temp_dir();
//! let result = extract(url, &archive, &out_dir).unwrap();
//! }
//! ```
//!
//! ## Feature flags
//!
//! postgresql_archive uses [feature flags] to address compile time and binary size
//! uses.
//!
//! The following features are available:
//!
//! | Name         | Description                | Default? |
//! |--------------|----------------------------|----------|
//! | `blocking`   | Enables the blocking API   | No       |
//! | `native-tls` | Enables native-tls support | Yes      |
//! | `rustls-tls` | Enables rustls-tls support | No       |
//!
//! ### Configurations
//!
//! | Name      | Description                         | Default? |
//! |-----------|-------------------------------------|----------|
//! | `theseus` | Enables theseus PostgreSQL binaries | Yes      |
//! | `zonky`   | Enables zonky PostgreSQL binaries   | No       |
//!
//! ### Hashers
//!
//! | Name   | Description          | Default? |
//! |--------|----------------------|----------|
//! | `md5`  | Enables md5 hashers  | No       |
//! | `sha1` | Enables sha1 hashers | No       |
//! | `sha2` | Enables sha2 hashers | Yes¹     |
//!
//! ¹ enabled by the `theseus` feature flag.
//!
//! ### Repositories
//!
//! | Name     | Description               | Default? |
//! |----------|---------------------------|----------|
//! | `github` | Enables github repository | Yes¹     |
//! | `maven`  | Enables maven repository  | No       |
//!
//! ¹ enabled by the `theseus` feature flag.
//!
//! ## Supported platforms
//!
//! `postgresql_archive` provides implementations for the following:
//!
//! * [theseus-rs/postgresql-binaries](https://github.com/theseus-rs/postgresql-binaries)
//! * [zonkyio/embedded-postgres-binaries](https://github.com/zonkyio/embedded-postgres-binaries)
//!
//! ## Safety
//!
//! This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.
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

#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![deny(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::module_name_repetitions)]

mod archive;
#[cfg(feature = "blocking")]
pub mod blocking;
pub mod configuration;
mod error;
pub mod extractor;
pub mod hasher;
pub mod matcher;
pub mod repository;
mod version;

pub use archive::{extract, get_archive, get_version};
pub use error::{Error, Result};
pub use semver::{Version, VersionReq};
pub use version::{ExactVersion, ExactVersionReq};
