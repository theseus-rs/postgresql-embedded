//! # postgresql_embedded
//!
//! [![Crates.io](https://img.shields.io/crates/v/postgresql_embedded)](http://crates.io/crates/postgresql_embedded)
//! [![Docs.rs](https://docs.rs/postgresql_embedded/badge.svg)](https://docs.rs/postgresql_embedded)
//! [![Crates.io](https://img.shields.io/crates/d/postgresql_embedded)](http://crates.io/crates/postgresql_embedded)
//! [![Crates.io](https://img.shields.io/crates/l/postgresql_embedded)](https://github.com/theseus-rs/postgresql_embedded/blob/main/postgresql_embedded/LICENSE)
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
//! use postgresql_embedded::PostgreSQL;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut postgresql = PostgreSQL::default();
//!     postgresql.setup().await.unwrap();
//!     postgresql.start().await.unwrap();
//!
//!     let database_name = "test";
//!     postgresql.create_database(database_name).await.unwrap();
//!     postgresql.database_exists(database_name).await.unwrap();
//!     postgresql.drop_database(database_name).await.unwrap();
//!
//!     postgresql.stop().await.unwrap();
//! }
//! ```
//!
//! ### Synchronous API
//! ```no_run
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
//! postgresql_embedded uses [feature flags] to address compile time and binary size
//! uses.
//!
//! The following features are available:
//!
//! | Name       | Description                                               | Default? |
//! |------------|-----------------------------------------------------------|----------|
//! | `bundled`  | Bundles the PostgreSQL archive into the resulting binary  | Yes      |
//! | `blocking` | Enables the blocking API; requires `tokio`                | No       |
//! | `tokio`    | Enables using tokio for async                             | No       |
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
#![allow(dead_code)]

#[cfg(feature = "blocking")]
pub mod blocking;
mod command;
mod error;
mod postgresql;
mod settings;

pub use error::{EmbeddedError, Result};
pub use postgresql::{PostgreSQL, Status};
pub use settings::Settings;
