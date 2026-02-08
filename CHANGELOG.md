# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## `postgresql_extensions` - [0.20.1](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_extensions-v0.20.0...postgresql_extensions-v0.20.1) - 2026-02-08

### Other
- update rust to 1.92.0
- reduce map_err by adding some From<Error> implementations
- reduce map_err by adding some From<Error> implementations

## `postgresql_embedded` - [0.20.1](https://github.com/theseus-rs/postgresql-embedded/compare/v0.20.0...v0.20.1) - 2026-02-08

### Added
- add postgresql v18 support

### Fixed
- update to support all targets

### Other
- Merge branch 'main' into caching_builds
- Target
- Cache archives
- update rust to 1.92.0
- Merge pull request #222 from gazure/ga/refactor-error-from-impls
- reduce map_err by adding some From<Error> implementations
- reduce map_err by adding some From<Error> implementations

## `postgresql_commands` - [0.20.1](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_commands-v0.20.0...postgresql_commands-v0.20.1) - 2026-02-08

### Other
- update rust to 1.92.0

## `postgresql_archive` - [0.20.1](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_archive-v0.20.0...postgresql_archive-v0.20.1) - 2026-02-08

### Other
- update rust to 1.92.0
- Merge pull request #222 from gazure/ga/refactor-error-from-impls
- reduce map_err by adding some From<Error> implementations
- reduce map_err by adding some From<Error> implementations

## `postgresql_extensions` - [0.20.0](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_extensions-v0.19.0...postgresql_extensions-v0.20.0) - 2025-08-31

### Fixed
- always use the build version of postgresql when the bundled feature is enabled to avoid network access

### Other
- remove devcontainer support

## `postgresql_embedded` - [0.20.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.19.0...v0.20.0) - 2025-08-31

### Fixed
- always use the build version of postgresql when the bundled feature is enabled to avoid network access
- [**breaking**] rename pg_dump compression argument to compress

### Other
- minor doc updates
- remove devcontainer support
- correct lint errors
- update to Rust 1.89.0

## `postgresql_commands` - [0.20.0](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_commands-v0.19.0...postgresql_commands-v0.20.0) - 2025-08-31

### Fixed
- [**breaking**] rename pg_dump compression argument to compress

### Other
- remove devcontainer support

## `postgresql_archive` - [0.20.0](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_archive-v0.19.0...postgresql_archive-v0.20.0) - 2025-08-31

### Other
- minor doc updates
- remove devcontainer support

## `postgresql_embedded` - [0.19.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.18.7...v0.19.0) - 2025-06-24

### Added
- allow skipping the installation step during setup

### Other
- correct typo in variable name
- update extractor feature documentation

## `postgresql_archive` - [0.19.0](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_archive-v0.18.7...postgresql_archive-v0.19.0) - 2025-06-24

### Other
- update extractor feature documentation

## `postgresql_embedded` - [0.18.7](https://github.com/theseus-rs/postgresql-embedded/compare/v0.18.6...v0.18.7) - 2025-06-20

### Fixed
- set CREATE_NO_WINDOW creation flag on Windows

### Other
- update Cargo.toml dependencies

## `postgresql_commands` - [0.18.7](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_commands-v0.18.6...postgresql_commands-v0.18.7) - 2025-06-20

### Fixed
- set CREATE_NO_WINDOW creation flag on Windows

## `postgresql_archive` - [0.18.7](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_archive-v0.18.6...postgresql_archive-v0.18.7) - 2025-06-20

### Other
- update Cargo.toml dependencies

## `postgresql_extensions` - [0.18.6](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_extensions-v0.18.5...postgresql_extensions-v0.18.6) - 2025-06-17

### Added

- add extractor feature flags

### Other

- correct lint errors

## `postgresql_embedded` - [0.18.6](https://github.com/theseus-rs/postgresql-embedded/compare/v0.18.5...v0.18.6) - 2025-06-17

### Added

- add extractor feature flags

### Other

- make liblzma an optional dependency
- add documentation for bundled feature flag
- correct lint errors

## `postgresql_archive` - [0.18.6](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_archive-v0.18.5...postgresql_archive-v0.18.6) - 2025-06-17

### Added

- add extractor feature flags

### Other

- make liblzma an optional dependency
- correct lint errors

## `postgresql_extensions` - [0.18.5](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_extensions-v0.18.4...postgresql_extensions-v0.18.5) - 2025-05-28

### Other
- update Cargo.toml dependencies

## `postgresql_embedded` - [0.18.5](https://github.com/theseus-rs/postgresql-embedded/compare/v0.18.4...v0.18.5) - 2025-05-28

### Fixed
- correct theseus build bundle
- revert SupportFn type change
- custom release url not working and compilation failure

### Other
- Merge branch 'main' into main
- update to criterion=0.6.0, pgvector=0.4.1, reqwest=0.12.18, sqlx=0.8.6, tokio=1.45.1, zip=4.0.0
- minor syntax change
- update Cargo.toml dependencies

## `postgresql_commands` - [0.18.5](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_commands-v0.18.4...postgresql_commands-v0.18.5) - 2025-05-28

### Other
- update Cargo.toml dependencies

## `postgresql_archive` - [0.18.5](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_archive-v0.18.4...postgresql_archive-v0.18.5) - 2025-05-28

### Fixed
- correct theseus build bundle
- revert SupportFn type change
- custom release url not working and compilation failure

### Other
- update to criterion=0.6.0, pgvector=0.4.1, reqwest=0.12.18, sqlx=0.8.6, tokio=1.45.1, zip=4.0.0
- minor syntax change

## `postgresql_extensions` - [0.18.4](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_extensions-v0.18.3...postgresql_extensions-v0.18.4) - 2025-05-15

### Other
- update Cargo.toml dependencies

## `postgresql_embedded` - [0.18.4](https://github.com/theseus-rs/postgresql-embedded/compare/v0.18.3...v0.18.4) - 2025-05-15

### Other
- update to Rust 1.87.0
- update dependencies
- update Cargo.toml dependencies

## `postgresql_commands` - [0.18.4](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_commands-v0.18.3...postgresql_commands-v0.18.4) - 2025-05-15

### Other
- update to Rust 1.87.0

## `postgresql_archive` - [0.18.4](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_archive-v0.18.3...postgresql_archive-v0.18.4) - 2025-05-15

### Other
- update to Rust 1.87.0
- update dependencies

## `postgresql_extensions` - [0.18.3](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_extensions-v0.18.2...postgresql_extensions-v0.18.3) - 2025-04-03

### Other
- update to Rust 1.86.0

## `postgresql_embedded` - [0.18.3](https://github.com/theseus-rs/postgresql-embedded/compare/v0.18.2...v0.18.3) - 2025-04-03

### Other
- update Cargo.toml dependencies
- update to Rust 1.86.0

## `postgresql_archive` - [0.18.3](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_archive-v0.18.2...postgresql_archive-v0.18.3) - 2025-04-03

### Other
- update Cargo.toml dependencies

## `postgresql_extensions` - [0.18.2](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_extensions-v0.18.1...postgresql_extensions-v0.18.2) - 2025-03-21

### Other
- update Cargo.toml dependencies

## `postgresql_embedded` - [0.18.2](https://github.com/theseus-rs/postgresql-embedded/compare/v0.18.1...v0.18.2) - 2025-03-21

### Other
- update Cargo.toml dependencies

## `postgresql_commands` - [0.18.2](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_commands-v0.18.1...postgresql_commands-v0.18.2) - 2025-03-21

### Other
- update Cargo.toml dependencies

## `postgresql_archive` - [0.18.2](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_archive-v0.18.1...postgresql_archive-v0.18.2) - 2025-03-21

### Other
- update Cargo.toml dependencies

## `postgresql_embedded` - [0.18.1](https://github.com/theseus-rs/postgresql-embedded/compare/v0.18.0...v0.18.1) - 2025-02-26

### Fix
- Check for existing installations in children before installing

## `postgresql_extensions` - [0.18.0](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_extensions-v0.17.5...postgresql_extensions-v0.18.0) - 2025-02-20

### Added
- update to Rust 2024 edition

### Other
- [**breaking**] rename feature rustls-tls to rustls

## `postgresql_commands` - [0.18.0](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_commands-v0.17.5...postgresql_commands-v0.18.0) - 2025-02-20

### Added
- update to Rust 2024 edition

## `postgresql_embedded` - [0.18.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.17.5...v0.18.0) - 2025-02-20

### Added
- update to Rust 2024 edition

### Other
- update dependencies
- [**breaking**] rename feature rustls-tls to rustls

## `postgresql_archive` - [0.18.0](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_archive-v0.17.5...postgresql_archive-v0.18.0) - 2025-02-20

### Added
- update to Rust 2024 edition

### Other
- [**breaking**] rename feature rustls-tls to rustls

## `postgresql_extensions` - [0.17.5](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_extensions-v0.17.4...postgresql_extensions-v0.17.5) - 2025-01-25

### Other
- replace regex with regex-lite to reduce dependencies
- update ci configuration

## `postgresql_commands` - [0.17.5](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_commands-v0.17.4...postgresql_commands-v0.17.5) - 2025-01-25

### Other
- remove anyhow and human_bytes dependencies

## `postgresql_embedded` - [0.17.5](https://github.com/theseus-rs/postgresql-embedded/compare/v0.17.4...v0.17.5) - 2025-01-25

### Other
- make tracing-indicatif optional
- remove anyhow and human_bytes dependencies
- replace regex with regex-lite to reduce dependencies
- remove http dependency
- update ci configuration

## `postgresql_archive` - [0.17.5](https://github.com/theseus-rs/postgresql-embedded/compare/postgresql_archive-v0.17.4...postgresql_archive-v0.17.5) - 2025-01-25

### Other
- replace regex with regex-lite to reduce dependencies
- remove http dependency
- make tracing-indicatif optional
- remove anyhow and human_bytes dependencies

## `postgresql_embedded` - [v0.17.4](https://github.com/theseus-rs/postgresql-embedded/compare/v0.17.3...v0.17.4) - 2025-01-17

### Chore

- update to Rust 1.83
- update to Rust 1.84

### Fix

- correct deny.toml
- use tokio::process::spawn() for pc_ctl on Windows

## `postgresql_embedded` - [v0.17.3](https://github.com/theseus-rs/postgresql-embedded/compare/v0.17.2...v0.17.3) - 2024-11-12

### Build

- update codecov action to version 4
- update code coverage generation
- update to Rust 1.82.0

### Chore

- add FUNDING.yml
- add FUNDING.yml
- correct new linting errors
- update dependencies
- add Unicode-3.0 as an allowed license

### Fix

- correct zonky extractor

## `postgresql_embedded` - [v0.17.2](https://github.com/theseus-rs/postgresql-embedded/compare/v0.17.1...v0.17.2) - 2024-10-01

### Build

- correct documentation build

## `postgresql_embedded` - [v0.17.1](https://github.com/theseus-rs/postgresql-embedded/compare/v0.17.0...v0.17.1) - 2024-10-01

### Build

- correct documentation build
- update dependencies

## `postgresql_embedded` - [v0.17.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.16.3...v0.17.0) - 2024-09-28

### Chore

- update dependencies
- add issue templates
- forbid clippy allow attributes
- add rust-toolchain.toml
- updates for clippy lints

### Deprecated

- [**breaking**] remove version 12 and deprecate version 13

### Fix

- allow archives to be bundled from alternate github repositories

### Test

- update extension test to run with specific postgresql version

## `postgresql_embedded` - [v0.16.3](https://github.com/theseus-rs/postgresql-embedded/compare/v0.16.2...v0.16.3) - 2024-09-04

### Chore

- switch from xz2 to liblzma
- ignore .idea directory

## `postgresql_embedded` - [v0.16.2](https://github.com/theseus-rs/postgresql-embedded/compare/v0.16.1...v0.16.2) - 2024-08-24

### Build

- update audit and deny checks

### Docs

- split axum and progress bar examples
- minor doc correction

### Fix

- update dependencies to address [RUSTSEC-2024-0363](https://rustsec.org/advisories/RUSTSEC-2024-0363.html)

### Refactor

- rename embedded_async_diesel_r2d2 to diesel_embedded

## `postgresql_embedded` - [v0.16.1](https://github.com/theseus-rs/postgresql-embedded/compare/v0.16.0...v0.16.1) - 2024-08-13

### Build

- remove unused dependencies

### Docs

- add axum example
- add indicatif to axum example

### Feat

- add archive tracing progress bar status

### Fix

- update maven status to set progress bar position

### Test

- update version of postgresql used for testing from 16.3.0 to 16.4.0
- update windows test assertion

## `postgresql_embedded` - [v0.16.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.15.0...v0.16.0) - 2024-08-04

### Build

- sort dependencies
- update dependencies
- address lint error

### Docs

- add PortalCorp example for pgvector

### Feat

- add portal corp extensions

### Fix

- correct steampipe extension url resolution
- add .dll support
- update steampipe to use detected OS if not on macos
- correct extension regex to match file extensions

### Refactor

- [**breaking**] refactor extension matchers
- [**breaking**] return list of files from archive extract function
- [**breaking**] refactor archive extract directories
- refactor zonky extractor to use generic tar_xz_extractor

### Test

- update portal corp test so that it does not run on macos x64
- add tests for extension matchers
- update archive test assertions to be platform specific
- update expected files extracted
- improve matcher error tests
- enable portal corp test for all platforms

## `postgresql_embedded` - [v0.15.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.14.2...v0.15.0) - 2024-08-01

### Build

- update tls features
- add github feature to steampipe and tensor-chord

### Docs

- correct doc errors
- correct doc errors
- add vector extension example
- update vector_extension example to run queries

### Feat

- [**breaking**] initial postgresql_extensions crate

### Fix

- registered github archive repositories for extensions
- correct steampipe install matcher
- [**breaking**] update extension matchers to use postgresql major version
- correct cargo check failure
- correct serialization error writing configuration
- correct vector example error
- linting error

### Refactor

- de-deduplicate steampipe matcher logic

### Test

- add version tests
- remove unused extension model
- update lifecycle test to run on linux only
- update steampipe test to run on macos
- disable steampipe test on macos
- update steampipe matcher test
- improve model test coverage

## `postgresql_embedded` - [v0.14.2](https://github.com/theseus-rs/postgresql-embedded/compare/v0.14.1...v0.14.2) - 2024-07-17

### Build

- remove clear caches action

### Docs

- add version optimization documentation

### Fix

- updated PgConfigBuilder interface to align with pg_config executable
- improve commands on windows to return stdout and stderr
- correct linting errors

### Test

- correct windows test failure

## `postgresql_embedded` - [v0.14.1](https://github.com/theseus-rs/postgresql-embedded/compare/v0.14.0...v0.14.1) - 2024-07-06

### Build

- change default from rustls-tls to native-tls
- suppress lint warning
- correct lint error
- correct formatting
- update non-windows build configuration
- update non-windows build tests

### Docs

- update docs for new features

### Fix

- correct bug where commands hang on windows when retrieving stdout/stderr
- correct hang when tokio is not used
- update command tests to work on Windows

### Test

- correct linux/macos tests
- increase timeout to 30 seconds
- increase timeout to 30 seconds
- revert timeout to 5 seconds

## `postgresql_embedded` - [v0.14.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.13.0...v0.14.0) - 2024-07-03

### Feat

- [**breaking**] add feature flags to enable zonky

### Test

- correct extract test implementations

## `postgresql_embedded` - [v0.13.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.12.0...v0.13.0) - 2024-07-01

### Build

- pin dependencies
- update use definitions when blocking feature enabled
- unpin dependencies
- correct url dependency definition
- correct documentation link error
- print target triple during build
- remove build caching
- correct lint error
- update license rules
- correct formatting error

### Docs

- update README.md
- simplify documentation
- remove reference to Bytes
- update documentation
- update readmes

### Feat

- [**breaking**] add semantic versioing support and configurable repositories
- add matcher registry
- [**breaking**] add configurable hashers
- add sha2-512 support
- add blake2 and sha3 hash support
- add hasher and matcher supports function
- [**breaking**] add configurable extractors
- add support for installing binaries from the zonky project
- add SHA1 hash support for older Maven repositories
- utilize sqlx for database management to support PostgreSQL installations that do not bundle psql
- update hasher registry to work with Maven central and add MD5 hash

### Fix

- correct asset hash logic
- convert possible panics to errors

### Refactor

- [**breaking**] rename ReleaseNotFound to VersionNotFound
- [**breaking**] remove bytes dependency
- [**breaking**] remove bytes dependency
- remove default registry values

### Test

- remove extraneous tests
- add tests to improve test coverage
- correct test_blake2b_512
- improve test coverage
- add zonky archive integration test
- correct hash test

## `postgresql_embedded` - [v0.12.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.11.0...v0.12.0) - 2024-06-21

### Refactor

- [**breaking**] move version from PostgreSQL::new() to Settings

## `postgresql_embedded` - [v0.11.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.10.2...v0.11.0) - 2024-06-20

### Build

- Enable pedantic lints

### Docs

- update documentation
- updated archive documentation examples

### Feat

- [**breaking**] allow releases URL to be configured
- allow releases url to be specified at build time when the bundles flag is set with the POSTGRESQL_RELEASES_URL
  environment variable
- export Version to improve dx

### Fix

- reference settings directly instead of via function call
- update examples
- pass settings release_url when bundled flag is set

### Test

- add missing command error tests and clean up lint directives

## `postgresql_embedded` - [v0.10.2](https://github.com/theseus-rs/postgresql-embedded/compare/v0.10.1...v0.10.2) - 2024-06-18

### Fix

- correct errors when PGDATABASE envar is set

## `postgresql_embedded` - [v0.10.1](https://github.com/theseus-rs/postgresql-embedded/compare/v0.9.5...v0.10.1) - 2024-06-14

### Build

- allow Unicode-3.0 license

### Feat

- [**breaking**] add ability to specify multiple pg_ctl options and define server configuration in Settings

## `postgresql_embedded` - [v0.9.5](https://github.com/theseus-rs/postgresql-embedded/compare/v0.9.4...v0.9.5) - 2024-06-03

### Build

- address pedantic clippy warnings

### Fix

- don't require rustls for the build script. only enable by default.

## `postgresql_embedded` - [v0.9.4](https://github.com/theseus-rs/postgresql-embedded/compare/v0.9.3...v0.9.4) - 2024-05-31

### Feat

- add native-tls support

## `postgresql_embedded` - [v0.9.3](https://github.com/theseus-rs/postgresql-embedded/compare/v0.9.2...v0.9.3) - 2024-05-21

### PostgreSQL

- don't trace self, and when tracing commands only trace the base name. makes the traces less enormous and also avoids
  dumping passwords into traces.

## `postgresql_embedded` - [v0.9.2](https://github.com/theseus-rs/postgresql-embedded/compare/v0.9.1...v0.9.2) - 2024-05-19

### Build

- correct lint warnings
- update dependencies

### Chore

- update dependencies

### Fix

- correct debug message

### Test

- add authentication tests
- improve test coverage

## `postgresql_embedded` - [v0.9.1](https://github.com/theseus-rs/postgresql-embedded/compare/v0.9.0...v0.9.1) - 2024-05-01

### Fix

- create extract_dir on same filesystem as out_dir

##
`postgresql_embedded` - [v0.9.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.8.3...v0.9.0) - 2024-04-26

### Fix

- [**breaking**] define bootstrap superuser as postgres
- [**breaking**] define bootstrap superuser as postgres

##
`postgresql_embedded` - [v0.8.3](https://github.com/theseus-rs/postgresql-embedded/compare/v0.8.2...v0.8.3) - 2024-04-21

### Build

- add CODECOV_TOKEN to code coverage build step

### Chore

- update dependencies
- update reqwest libraries
- address format error

## `postgresql_embedded` - [v0.8.2](https://github.com/theseus-rs/postgresql-embedded/compare/v0.8.1...v0.8.2) - 2024-04-05

### Fix

- suppress bytes parameter in tracing instrumentation

## `postgresql_embedded` - [v0.8.1](https://github.com/theseus-rs/postgresql-embedded/compare/v0.8.0...v0.8.1) - 2024-04-03

### Build

- update build dependencies to address audit check

### Test

- add command integration test

## `postgresql_embedded` - [v0.8.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.7.3...v0.8.0) - 2024-04-03

### Build

- update dependencies
- correct linting errors

### Refactor

- [**breaking**] move commands into postgresql_commands crate

## `postgresql_embedded` - [v0.7.3](https://github.com/theseus-rs/postgresql-embedded/compare/v0.7.2...v0.7.3) - 2024-03-25

### Chore

- remove scorecard.yml

### Feat

- add ability to create settings from a url

### Refact

- remove use of embedded=true parameter

## `postgresql_embedded` - [v0.7.2](https://github.com/theseus-rs/postgresql-embedded/compare/v0.7.1...v0.7.2) - 2024-03-16

### Chore

- add Debug trait to CommandBuilder

### Feat

- add tracing instrumentation

## `postgresql_embedded` - [v0.7.1](https://github.com/theseus-rs/postgresql-embedded/compare/v0.7.0...v0.7.1) - 2024-03-15

### Fix

- correct parallel archive extract failures

## `postgresql_embedded` - [v0.7.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.6.2...v0.7.0) - 2024-03-15

### Docs

- update vscode development container instructions

### Fix

- [**breaking**] correct parallel archive extract failures

## `postgresql_embedded` - [v0.6.2](https://github.com/theseus-rs/postgresql-embedded/compare/v0.6.1...v0.6.2) - 2024-03-07

### Chore

- correct lint error

### Feat

- add reqwest backoff/retry logic and tracing support

## `postgresql_embedded` - [v0.6.1](https://github.com/theseus-rs/postgresql-embedded/compare/v0.6.0...v0.6.1) - 2024-03-06

### Chore

- update use of settings of postgres connection and correct typo in output
- update dependencies
- remove use of copy left license MPL-2.0 from dependencies

### Fix

- update dependencies to address RUSTSEC-2024-0020

## `postgresql_embedded` - [v0.6.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.5.0...v0.6.0) - 2024-02-24

### Chore

- correct formatting
- correct linting error

### Fix

- [**breaking**] remove bundled as a default feature and corrected bug when the bundled feature is not used

## `postgresql_embedded` - [v0.5.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.4.1...v0.5.0) - 2024-02-22

### Chore

- remove unnecessary use of command pipes
- update action permissions to reduce write privilege scope
- ignore RUSTSEC-2023-0071 as it is only used in sqlx example code
- correct linting errors

### Ci

- run all benchmarks from workspace at once instead of individually from crates

### Docs

- add SECURITY.md
- add postgres driver and sqlx examples
- add documentation explaining why RUSTSEC-2023-0071 is ignored

### Refactor

- [**breaking**] refactor status to check on demand instead of attempting to track the state dynamically

### Test

- remove unused code

## `postgresql_embedded` - [v0.4.1](https://github.com/theseus-rs/postgresql-embedded/compare/v0.4.0...v0.4.1) - 2024-02-18

### Chore

- Add initial dev container support
- update windows to use UTF8 to align with other operating systems and utilize capabilities of the newer releases
  from https://github.com/theseus-rs/postgresql-binaries
- add code coverage configuration
- remove extraneous line in Cargo.toml
- update release drafter to version 6 to address node 16 deprecation warning
- update pr-benchmarks name

### Ci

- update build to run benchmarks
- add BENCHER_API_TOKEN to benchmark action
- remove build.yml and move jobs into ci.yml
- split benchmark runs
- update build to run benchmarks
- add benchmark pull request integration
- update approach for setting ci-number
- add pull-requests: write permission
- remove conditional pr benchmark statements

### Docs

- add cargo keywords
- update docs for new functions
- add bencher badges

### Feat

- add devcontainer support

### Refactor

- update psql to manage setting the PGPASSWORD environment variable when pg_password is set
- refactor the way benchmarks run on the main branch vs a PR

### Test

- add benchmarks
- add CommandBuilder test coverage
- correct the embedded lifecycle benchmark name
- reduce archive benchmark sample size to 10
- update benchmark configuration
- remove bencher conditional arguments
- combine benchmark runs into one step
- remove all bencher options
- reduce embedded sample size to 10 to reduce benchmark runtime
- update benchmark pull request configuration

## `postgresql_embedded` - [v0.4.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.3.2...v0.4.0) - 2024-02-13

### Docs

- add postgres to keywords

### Refactor

- [**breaking**] remove archive hash from the public interface and always calculate/verify the has when requesting an
  archive
- [**breaking**] remove archive hash from the public interface and always calculate/verify the has when requesting an
  archive
- simplified installation logic and improved code coverage

### Test

- improve lifecycle test coverage
- update elapsed error test to sleep longer to prevent intermittent test failure

## `postgresql_embedded` - [v0.3.2](https://github.com/theseus-rs/postgresql-embedded/compare/v0.3.1...v0.3.2) - 2024-02-13

### Bug

- correct bug where serialization fails when there is a draft release of the PostgreSQL binaries

### Chore

- add examples
- add missing license definitions

### Test

- update test code coverage
- add tests for examples

## `postgresql_embedded` - [v0.3.1](https://github.com/theseus-rs/postgresql-embedded/compare/v0.3.0...v0.3.1) - 2024-02-12

### Chore

- address linting error
- change tracing levels from info to debug

### Ci

- add pull request labeler

### Docs

- update cargo description

### Refactor

- update postgresql_embedded::ArchiveError argument

## `postgresql_embedded` - [v0.3.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.2.3...v0.3.0) - 2024-02-11

### Ci

- add release drafter

### Refactor

- [**breaking**] rename ArchiveError to postgresql_archive::Error and EmbeddedError to postgresql_embedded::Error

## `postgresql_embedded` - [v0.2.3](https://github.com/theseus-rs/postgresql-embedded/compare/v0.2.2...v0.2.3) - 2024-02-11

### Ci

- add scheduled action to clear github caches

## `postgresql_embedded` - [v0.2.2](https://github.com/theseus-rs/postgresql-embedded/compare/v0.2.1...v0.2.2) - 2024-02-11

### Bug

- warn when a release tag name does not match the expected version pattern

### Chore

- remove default feature test
- update release to 0.2.2

### Docs

- wrap synchronous API docs in feature blocks
- remove ci badge from rust docs
- update examples in documentation to remove unnecessary use of .unwrap()

### Feat

- enable code coverage
- add code coverage badges

### Test

- add tests to improve code coverage
- updated valid initial statuses

## `postgresql_embedded` - [v0.2.1](https://github.com/theseus-rs/postgresql-embedded/compare/v0.2.0...v0.2.1) - 2024-02-10

### Chore

- update release to 0.2.1

### Docs

- enable documentation features

## `postgresql_embedded` - [v0.2.0](https://github.com/theseus-rs/postgresql-embedded/compare/v0.1.2...v0.2.0) - 2024-02-10

### Chore

- update release to 0.2.0

### Docs

- updated examples to use no_run to prevent documentation build failures

## `postgresql_embedded` - [v0.1.2](https://github.com/theseus-rs/postgresql-embedded/compare/v0.1.1...v0.1.2) - 2024-02-10

### Chore

- remove cargo vet check
- remove unused cargo dist configuration
- update release to 0.1.2

### Docs

- update badges for release
- correct crate repository urls
- add documentation for CommandExecutor
- remove note regarding tokio usage for the example
- added documentation for POSTGRESQL_VERSION and GITHUB_TOKEN usage

## `postgresql_embedded` - [v0.1.1](https://github.com/theseus-rs/postgresql-embedded/compare/v0.1.0...v0.1.1) - 2024-02-10

### Docs

- mark docs as ignored to prevent doc release failures

## `postgresql_embedded` - [v0.1.0](https://github.com/theseus-rs/postgresql-embedded/compare/bd97bf1b5b53beb503034d499a0186c75ba6271e...v0.1.0) - 2024-02-10

### Bug

- corrected unused import and unused variable errors when building on windows
- update postgresql_embedded to enable "bundled" as a default feature
- correct doc lint
- correct command test failures on windows
- correct command builder test failures on windows
- correct command builder test bugs on windows
- update archive extract to support symlinks
- corrected extract bug on MacOS caused by a directory being treated as a file
- set encoding to SQL_ASCII for windows until binary is built with UTF8 support; use -o instead of --option when
  attempting to start the server
- remove failing code coverage actions

### Build

- *(deps)* bump tempfile from 3.9.0 to 3.10.0

### Chore

- initial CI configuration
- updated tempfile config for cargo vet
- reduce test execution and setup code coverage
- enable rust / cargo caching for ci
- enable caching to ci checks
- update vet check for hermit-abi
- update cargo vet config
- add GITHUB_TOKEN to clippy and tests to address rate limiting
- disable windows build
- add author and release metadata
- add missing crate descriptions
- update release metadata

### Docs

- update MIT License header
- update ci status badge
- disable blocking rust doc examples

### Feat

- add ability to embed PostgreSQL installation in a Rust executable
- add GITHUB_TOKEN as a Bearer token when calling the GitHub API in order to increase the rate limit
- added initial tracing support

### Refactor

- update the name of the postgresql binaries repository

### Test

- refactor version constant tests so that they can be run in parallel to speed up builds
- corrected pg_ctl test

