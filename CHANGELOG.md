# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# Unreleased

# 1.0.0 (7 Sept 2024)

- **changed:** BREAKING CHANGE: `read_limited()` returns `LimitReaderOutput` instead of a `usize`
- **changed:** Update README for running build script
- **changed:** Rename and improve info printed to STDOUT for `read_limited.rs` example.

# 0.2.0 (6 Sept 2024)

- **added:** Add Changelog
- **added:** Add Error type for crate
- **added:** Add `LimitReaderResult`, use as the default `Result` type.
- **changed:** Remove `anyhow`