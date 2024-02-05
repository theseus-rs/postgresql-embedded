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
//! **Note**: The following example requires the `tokio` runtime.
//!
//! ```rust
//! use postgresql_embedded::PostgreSQL;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut postgresql = PostgreSQL::default();
//!     postgresql.setup().await.unwrap();
//!     postgresql.start().await.unwrap();
//!     postgresql.create_database("database_name").await.unwrap();
//!     postgresql.database_exists("database_name").await.unwrap();
//!     postgresql.drop_database("database_name").await.unwrap();
//!     postgresql.stop().await.unwrap();
//! }
//! ```
//!
//! ### Synchronous API
//! ```rust
//! #[cfg(feature = "blocking")]
//! {
//! use postgresql_embedded::blocking::PostgreSQL;
//!
//! let mut postgresql = PostgreSQL::default();
//! postgresql.setup().unwrap();
//! postgresql.start().unwrap();
//! postgresql.create_database("database_name").unwrap();
//! postgresql.database_exists("database_name").unwrap();
//! postgresql.drop_database("database_name").unwrap();
//! postgresql.stop().unwrap();
//! }
//! ```
//!
//! ## Information
//!
//! The downloaded postgresql binaries are cached in the following directories:
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
//! Name | Description | Default?
//! ---|---|---
//! `bundled` | Bundles the PostgreSQL archive into the resulting binary | Yes
//! `blocking` | Enables the blocking API; requires `tokio` | No
//! `tokio` | Enables using tokio for async | No
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
//! Uses PostgreSQL binaries from [theseus-rs/postgresql-binaries](https://github.com/theseus-rs/postgresql_binaries).

#![forbid(unsafe_code)]
#![allow(dead_code)]

#[cfg(feature = "blocking")]
pub mod blocking;
mod command;
mod error;
pub mod postgresql;
mod settings;

pub use error::{EmbeddedError, Result};
pub use postgresql::{PostgreSQL, Status};
pub use settings::Settings;
