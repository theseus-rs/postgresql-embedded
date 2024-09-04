<p align="center"><img width="250" height="250" src="images/logo.png"></p>

# PostgreSQL Embedded

[![ci](https://github.com/theseus-rs/postgresql-embedded/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/postgresql-embedded/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/postgresql_embedded/badge.svg)](https://docs.rs/postgresql_embedded)
[![Code Coverage](https://codecov.io/gh/theseus-rs/postgresql-embedded/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/postgresql-embedded)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-postgresql-embedded)
[![Latest version](https://img.shields.io/crates/v/postgresql_embedded.svg)](https://crates.io/crates/postgresql_embedded)
[![License](https://img.shields.io/crates/l/postgresql_embedded)](https://github.com/theseus-rs/postgresql-embedded#license)
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
- semantic version resolution
- support for custom PostgreSQL archives / binaries
- ability to configure PostgreSQL startup options
- URL based configuration
- choice of native-tls vs rustls
- support for installing PostgreSQL extensions

## Getting Started

### Example

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

## Safety

These crates use `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

PostgreSQL is covered under [The PostgreSQL License](https://opensource.org/licenses/postgresql).

## Notes

Supports using PostgreSQL binaries from:

* [theseus-rs/postgresql-binaries](https://github.com/theseus-rs/postgresql-binaries) (default)
* [zonkyio/embedded-postgres-binaries](https://github.com/zonkyio/embedded-postgres-binaries)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

<a href="https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/theseus-rs/postgresql-embedded">
<img
  src="https://img.shields.io/static/v1?label=VSCode%20Development%20Container&logo=visualstudiocode&message=Open&color=orange"
  alt="VSCode Development Container"
/>
</a>
<br/>
<a href="https://github.dev/theseus-rs/postgresql-embedded">
<img
  src="https://img.shields.io/static/v1?label=GitHub%20Codespaces&logo=github&message=Open&color=orange"
  alt="GitHub Codespaces"
/>
</a>

## Prior Art

Projects that inspired this one:

* [zonkyio/embedded-postgres-binaries](https://github.com/zonkyio/embedded-postgres-binaries)
* [faokunega/pg-embed](https://github.com/faokunega/pg-embed)
