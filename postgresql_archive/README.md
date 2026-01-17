# PostgreSQL Archive

[![ci](https://github.com/theseus-rs/postgresql-embedded/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/postgresql-embedded/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/postgresql_archive/badge.svg)](https://docs.rs/postgresql_archive)
[![Code Coverage](https://codecov.io/gh/theseus-rs/postgresql-embedded/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/postgresql-embedded)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-postgresql-embedded)
[![Latest version](https://img.shields.io/crates/v/postgresql_archive.svg)](https://crates.io/crates/postgresql_archive)
[![License](https://img.shields.io/crates/l/postgresql_archive?)](https://github.com/theseus-rs/postgresql-embedded/tree/main/postgresql_archive#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

A configurable library for downloading and extracting PostgreSQL archives.

## Examples

### Asynchronous API

```rust
use postgresql_archive::{extract, get_archive, Result, VersionReq};
use postgresql_archive::configuration::theseus;

#[tokio::main]
async fn main() -> Result<()> {
    let url = theseus::URL;
    let (archive_version, archive) = get_archive(url, &VersionReq::STAR).await?;
    let out_dir = std::env::temp_dir();
    extract(url, &archive, &out_dir).await
}
```

### Synchronous API

```rust
use postgresql_archive::configuration::theseus;
use postgresql_archive::{Result, VersionReq};
use postgresql_archive::blocking::{extract, get_archive};

fn main() -> Result<()> {
    let url = theseus::URL;
    let (archive_version, archive) = get_archive(url, &VersionReq::STAR)?;
    let out_dir = std::env::temp_dir();
    extract(url, &archive, &out_dir)
}
```

## Feature flags

postgresql_archive uses [feature flags] to address compile time and binary size
uses.

The following features are available:

| Name         | Description                      | Default? |
|--------------|----------------------------------|----------|
| `blocking`   | Enables the blocking API         | No       |
| `indicatif`  | Enables tracing-indcatif support | No       |
| `native-tls` | Enables native-tls support       | Yes      |
| `rustls`     | Enables rustls support           | No       |

### Configurations

| Name      | Description                         | Default? |
|-----------|-------------------------------------|----------|
| `theseus` | Enables theseus PostgreSQL binaries | Yes      |
| `zonky`   | Enables zonky PostgreSQL binaries   | No       |

### Extractors

| Name     | Description              | Default? |
|----------|--------------------------|----------|
| `tar-gz` | Enables tar gz extractor | Yes      |
| `tar-xz` | Enables tar xz extractor | No       |
| `zip`    | Enables zip extractor    | No       |

### Hashers

| Name   | Description          | Default? |
|--------|----------------------|----------|
| `md5`  | Enables md5 hashers  | No       |
| `sha1` | Enables sha1 hashers | No       |
| `sha2` | Enables sha2 hashers | Yes¹     |

¹ enabled by the `theseus` feature flag.

### Repositories

| Name     | Description               | Default? |
|----------|---------------------------|----------|
| `github` | Enables github repository | Yes¹     |
| `maven`  | Enables maven repository  | No       |

¹ enabled by the `theseus` feature flag.

## Supported platforms

`postgresql_archive` provides implementations for the following:

* [theseus-rs/postgresql-binaries](https://github.com/theseus-rs/postgresql-binaries)
* [zonkyio/embedded-postgres-binaries](https://github.com/zonkyio/embedded-postgres-binaries)

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
