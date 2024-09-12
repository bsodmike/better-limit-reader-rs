# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# Unreleased

- **changed:** CHANGELOG updated to reflect previously yanked versions.

# 3.0.0 (8 Sept 2024)

- **fixed:** Impl `std::error::Error` for the crates default error type.

# 2.0.0 (8 Sept 2024) - Yanked

- **added:** Github CI
- **fixed:** Clippy offences
- **changed:** Improve docs
- **changed:** Stablised return values to `u64`. Use of `usize` will need to be re-evaluated for future `no-std` support.

See [PR for changes](https://github.com/bsodmike/better-limit-reader-rs/pull/1/files)

# 1.0.2 (7 Sept 2024) - Yanked

- **changed:** Add enhancement task to README

# 1.0.1 (7 Sept 2024) - Yanked

- **changed:** Fix README with correct links to docs

# 1.0.0 (7 Sept 2024) - Yanked

- **changed:** BREAKING CHANGE: `read_limited()` returns `LimitReaderOutput` instead of a `usize`
- **changed:** Update README for running build script
- **changed:** Rename and improve info printed to STDOUT for `read_limited.rs` example.

# 0.2.0 (6 Sept 2024) - Yanked

- **added:** Add Changelog
- **added:** Add Error type for crate
- **added:** Add `LimitReaderResult`, use as the default `Result` type.
- **changed:** Remove `anyhow`