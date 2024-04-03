# PostgreSQL Commands

[![ci](https://github.com/theseus-rs/postgresql-embedded/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/postgresql-embedded/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/postgresql_commands/badge.svg)](https://docs.rs/postgresql_commands)
[![Code Coverage](https://codecov.io/gh/theseus-rs/postgresql-embedded/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/postgresql-embedded)
[![Latest version](https://img.shields.io/crates/v/postgresql_commands.svg)](https://crates.io/crates/postgresql_commands)
[![License](https://img.shields.io/crates/l/postgresql_commands?)](https://github.com/theseus-rs/postgresql-embedded/tree/main/postgresql_commands#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

A library for executing PostgreSQL command line utilities.

## Examples

```rust
use postgresql_commands::psql::PsqlBuilder;

let psql = PsqlBuilder::new()
.command("CREATE DATABASE \"test\"")
.host("127.0.0.1")
.port(5432)
.username("postgresql")
.pg_password("password")
.build();

let (stdout, stderr) = psql.execute(10).await?;
```

## Feature flags

postgresql_commands uses [feature flags] to address compile time and binary size
uses.

The following features are available:

| Name    | Description                       | Default? |
|---------|-----------------------------------|----------|
| `tokio` | Enables the use of tokio commands | No       |

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
