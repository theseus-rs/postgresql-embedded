# PostgreSQL Archive

[![ci](https://github.com/theseus-rs/postgresql-embedded/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/postgresql-embedded/actions/workflows/ci.yml)
[![Latest version](https://img.shields.io/crates/v/postgresql_archive.svg)](https://crates.io/crates/postgresql_embedded)
[![Documentation](https://docs.rs/postgresql_archive/badge.svg)](https://docs.rs/postgresql_embedded)
[![License](https://img.shields.io/crates/l/postgresql_archive?)](https://github.com/theseus-rs/postgresql-embedded/tree/main/postgresql_archive#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

---

A library for downloading and extracting PostgreSQL archives from
[theseus-rs/postgresql-binaries](https://github.com/theseus-rs/postgresql-binaries).

## Examples

### Asynchronous API

```rust
use postgresql_archive::{extract, get_archive, LATEST};

#[tokio::main]
async fn main() {
    let (archive_version, archive, hash) = get_archive(&LATEST).await.unwrap();
    let out_dir = std::env::temp_dir();
    let result = extract(&archive, &out_dir).await;
}
```

### Synchronous API
```rust
use postgresql_archive::LATEST;
use postgresql_archive::blocking::{extract, get_archive};

fn main() {
    let (archive_version, archive, hash) = get_archive(&LATEST).unwrap();
    let out_dir = std::env::temp_dir();
    let result = extract(&archive, &out_dir);
}
```

## Feature flags

postgresql_archive uses [feature flags] to address compile time and binary size
uses.

The following features are available:

Name | Description | Default?
---|---|---
`blocking` | Enables the blocking API | No

## Supported platforms

`postgresql_archive` supports all platforms provided by [theseus-rs/postgresql-binaries](https://github.com/theseus-rs/postgresql-binaries).

Currently supported platforms are:

OS | [Target](https://doc.rust-lang.org/nightly/rustc/platform-support.html)
---|---
Linux | aarch64-unknown-linux-gnu
Linux | aarch64-unknown-linux-musl
Linux | arm-unknown-linux-gnueabi
Linux | arm-unknown-linux-gnueabihf
Linux | arm-unknown-linux-musleabi
Linux | arm-unknown-linux-musleabihf
Linux | armv5te-unknown-linux-gnueabi
Linux | armv7-unknown-linux-gnueabihf
Linux | armv7-unknown-linux-musleabihf
Linux | i586-unknown-linux-gnu
Linux | i586-unknown-linux-musl
Linux | i686-unknown-linux-gnu
Linux | i686-unknown-linux-musl
Linux | mips64-unknown-linux-gnuabi64
Linux | powerpc64le-unknown-linux-gnu
Linux | powerpc64le-unknown-linux-musl
Linux | s390x-unknown-linux-gnu
Linux | s390x-unknown-linux-musl
Linux | x86_64-unknown-linux-gnu
Linux | x86_64-unknown-linux-musl
MacOS | aarch64-apple-darwin
MacOS | x86_64-apple-darwin
Windows | x86_64-pc-windows-msvc

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

PostgreSQL is covered under [The PostgreSQL License](https://opensource.org/licenses/postgresql).

## Notes

Uses PostgreSQL binaries from [theseus-rs/postgresql-binaries](https://github.com/theseus-rs/postgresql-binaries).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
