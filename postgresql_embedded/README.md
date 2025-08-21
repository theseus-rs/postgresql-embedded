# PostgreSQL Embedded

[![ci](https://github.com/theseus-rs/postgresql-embedded/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/postgresql-embedded/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/postgresql_embedded/badge.svg)](https://docs.rs/postgresql_embedded)
[![Code Coverage](https://codecov.io/gh/theseus-rs/postgresql-embedded/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/postgresql-embedded)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-postgresql-embedded)
[![Latest version](https://img.shields.io/crates/v/postgresql_embedded.svg)](https://crates.io/crates/postgresql_embedded)
[![License](https://img.shields.io/crates/l/postgresql_embedded)](https://github.com/theseus-rs/postgresql-embedded/tree/main/postgresql_embedded#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

Install and run a PostgreSQL database locally on Linux, MacOS or Windows. PostgreSQL can be
bundled with your application, or downloaded on demand.

This library provides an embedded-like experience for PostgreSQL similar to what you would have with
SQLite. This is accomplished by downloading and installing PostgreSQL during runtime. There is
also a "bundled" feature that when enabled, will download the PostgreSQL installation archive at
compile time, include it in your binary and install from the binary version at runtime.
In either case, PostgreSQL will run in a separate process space.

## Features

- installing and running PostgreSQL
- running PostgreSQL on ephemeral ports
- async and blocking API
- bundling the PostgreSQL archive in an executable
- dynamic version resolution
- ability to configure PostgreSQL startup options
- URL based configuration
- choice of native-tls vs rustls

## Examples

### Asynchronous API

```rust
use postgresql_embedded::{PostgreSQL, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut postgresql = PostgreSQL::default();
    postgresql.setup().await?;
    postgresql.start().await?;

    let database_name = "test";
    postgresql.create_database(database_name).await?;
    postgresql.database_exists(database_name).await?;
    postgresql.drop_database(database_name).await?;

    postgresql.stop().await
}
```

### Synchronous API

```rust
use postgresql_embedded::Result;
use postgresql_embedded::blocking::PostgreSQL;

fn main() -> Result<()> {
    let mut postgresql = PostgreSQL::default();
    postgresql.setup()?;
    postgresql.start()?;

    let database_name = "test";
    postgresql.create_database(database_name)?;
    postgresql.database_exists(database_name)?;
    postgresql.drop_database(database_name)?;

    postgresql.stop()
}
```

## Information

During the build process, when the `bundled` feature is enabled, the PostgreSQL binaries are
downloaded and included in the resulting binary. The version of the PostgreSQL binaries is
determined by the `POSTGRESQL_VERSION` environment variable. If the `POSTGRESQL_VERSION`
environment variable is not set, then `postgresql_archive::LATEST` will be used to determine the
version of the PostgreSQL binaries to download.

When downloading the theseus PostgreSQL binaries, either during build, or at runtime, the
`GITHUB_TOKEN` environment variable can be set to a GitHub personal access token to increase
the rate limit for downloading the PostgreSQL binaries. The `GITHUB_TOKEN` environment
variable is not required.

At runtime, the PostgreSQL binaries are cached by default in the following directories:

- Unix: `$HOME/.theseus/postgresql`
- Windows: `%USERPROFILE%\.theseus\postgresql`

Performance can be improved by using a specific version of the PostgreSQL binaries (e.g. `=16.4.0`).
After the first download, the PostgreSQL binaries will be cached and reused for subsequent runs.
Further, the repository will no longer be queried to calculate the version match.

## Feature flags

postgresql_embedded uses feature flags to address compile time and binary size
uses.

The following features are available:

| Name         | Description                                              | Default? |
|--------------|----------------------------------------------------------|----------|
| `bundled`    | Bundles the PostgreSQL archive into the resulting binary | No       |
| `blocking`   | Enables the blocking API; requires `tokio`               | No       |
| `indicatif`  | Enables tracing-indcatif support                         | No       |
| `native-tls` | Enables native-tls support                               | Yes      |
| `rustls`     | Enables rustls support                                   | No       |
| `theseus`    | Enables theseus PostgreSQL binaries                      | Yes      |
| `tokio`      | Enables using tokio for async                            | No       |
| `zonky`      | Enables zonky PostgreSQL binaries                        | No       |

## Bundling PostgreSQL

To bundle PostgreSQL with your application, you can enable the `bundled` feature. This will download the PostgreSQL
archive at compile time and include it in your binary. You should specify the version of PostgreSQL to bundle by
setting the environment variable `POSTGRESQL_VERSION` to a specific version, e.g. `=17.2.0`. In order to use the bundled
PostgreSQL, you will also need to set an explicit matching version at runtime in `Settings`:

```rust
use postgresql_embedded::{Result, Settings, VersionReq};

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings {
        version: VersionReq::from_str("=17.2.0")?,
        ..Default::default()
    };
    Ok(())
}
```

The PostgreSQL binaries can also be obtained from a different GitHub source by setting the `POSTGRESQL_RELEASES_URL`
environment variable. The repository must contain the releases with archives in same structure as
[theseus-rs/postgresql_binaries](https://github.com/theseus-rs/postgresql-binaries).

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Notes

Uses PostgreSQL binaries from [theseus-rs/postgresql-binaries](https://github.com/theseus-rs/postgresql_binaries) by
default.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
