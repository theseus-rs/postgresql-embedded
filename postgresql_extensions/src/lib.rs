//! # PostgreSQL Extensions
//!
//! [![ci](https://github.com/theseus-rs/postgresql-embedded/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/postgresql-embedded/actions/workflows/ci.yml)
//! [![Documentation](https://docs.rs/postgresql_extensions/badge.svg)](https://docs.rs/postgresql_extensions)
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/postgresql-embedded/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/postgresql-embedded)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-postgresql-embedded)
//! [![Latest version](https://img.shields.io/crates/v/postgresql_extensions.svg)](https://crates.io/crates/postgresql_extensions)
//! [![License](https://img.shields.io/crates/l/postgresql_extensions?)](https://github.com/theseus-rs/postgresql-embedded/tree/main/postgresql_extensions#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! A configurable library for managing PostgreSQL extensions.
//!
//! ## Examples
//!
//! ### Asynchronous API
//!
//! ```rust
//! use postgresql_extensions::{get_available_extensions, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let extensions = get_available_extensions().await?;
//!     Ok(())
//! }
//! ```
//!
//! ### Synchronous API
//!
//! ```rust
//! #[cfg(feature = "blocking")] {
//! use postgresql_extensions::Result;
//! use postgresql_extensions::blocking::get_available_extensions;
//!
//! let extensions = get_available_extensions().unwrap();
//! }
//! ```
//!
//! ## Feature flags
//!
//! postgresql_extensions uses [feature flags] to address compile time and binary size
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
//! ### Repositories
//!
//! | Name           | Description                               | Default? |
//! |----------------|-------------------------------------------|----------|
//! | `portal-corp`  | Enables PortalCorp PostgreSQL extensions  | Yes      |
//! | `steampipe`    | Enables Steampipe PostgreSQL extensions   | Yes      |
//! | `tensor-chord` | Enables TensorChord PostgreSQL extensions | Yes      |
//!
//! ## Supported platforms
//!
//! `postgresql_extensions` provides implementations for the following:
//!
//! * [steampipe/repositories](https://github.com/orgs/turbot/repositories)
//! * [tensor-chord/pgvecto.rs](https://github.com/tensor-chord/pgvecto.rs)
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
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
//! additional terms or conditions.

#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![deny(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::module_name_repetitions)]

#[cfg(feature = "blocking")]
pub mod blocking;
mod error;
pub mod extensions;
mod matcher;
mod model;
pub mod repository;

pub use error::{Error, Result};
pub use extensions::{get_available_extensions, get_installed_extensions, install, uninstall};
pub use matcher::{matcher, tar_gz_matcher, zip_matcher};
#[cfg(test)]
pub use model::TestSettings;
pub use model::{AvailableExtension, InstalledConfiguration, InstalledExtension};
pub use semver::{Version, VersionReq};
