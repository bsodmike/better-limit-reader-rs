# better-limit-reader-rs

![Crates.io Version](https://img.shields.io/crates/v/better-limit-reader)
[![Released API docs](https://img.shields.io/docsrs/better-limit-reader)](https://docs.rs/better-limit-reader/)

Exposes LimitReader which is a limit reader, that protects against zip-bombs and other nefarious activities that limits the number of bytes read from an underlying reader.

This crate is heavily inspired by Jon Gjengset’s “Crust of Rust” episode on the [inner workings of git on YouTube](https://youtu.be/u0VotuGzD_w?si=oIuV9CITSWHJXKBu&t=3503) and mitigrating Zip-bombs.

## API usage

Refer to the [docs](https://docs.rs/TODO) for further examples.

### Upcoming enhancements

- TBD: If you have any requests, please open an issue!

### Building

```shell script
cargo build --release
```

# MSRV

This project is tested against the most [recent stable rust version](https://gist.github.com/alexheretic/d1e98d8433b602e57f5d0a9637927e0c).

# License

This work is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.