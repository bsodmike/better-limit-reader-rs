#![warn(missing_docs)]

//! # better-limit-reader
//!
//! Exposes [`LimitReader`] which is a limit reader, that protects against zip-bombs and other nefarious activities.
//!
//! This crate is heavily inspired by Jon Gjengset's "Crust of Rust" episode on the inner workings of git on `YouTube` (<https://youtu.be/u0VotuGzD_w?si=oIuV9CITSWHJXKBu&t=3503>) and mitigrating Zip-bombs.

use derive_builder::Builder;
use error::LimitReaderError;
use flate2::read::ZlibDecoder;
use readable::MyBufReader;
use readable::Readable;
use readable::{falible::LimitReaderFallible, infalible::LimitReaderInfallible};
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

use LimitReaderResult as Result;

pub(crate) mod error;
pub(crate) mod readable;

/// Default result type for [`LimitReader`]
pub type LimitReaderResult<T> = std::result::Result<T, LimitReaderError>;

/// Re-exports Traits and macros used by most projects. Add `use better_limit_reader::prelude::*;` to your code to quickly get started with [`LimitReader`].
pub mod prelude {

    pub use crate::{error::LimitReaderError, LimitReader, LimitReaderOutput, LimitReaderResult};
}

#[allow(dead_code)]
/// The [`LimitReader`] reads into `buf` which is held within the record struct.
pub struct LimitReader {
    buf: [u8; Self::DEFAULT_BUF_SIZE],
    expected_size: u64,
    decode_zlib: bool,
    decode_gzip: bool,
}

impl Default for LimitReader {
    fn default() -> Self {
        Self::new()
    }
}

// Holds a `LimitReader` with a default buffer of size `LimitReader::DEFAULT_BUF_SIZE`
impl LimitReader {
    /// Default buffer size for the internal `LimitReader`
    pub const DEFAULT_BUF_SIZE: usize = 1024;

    /// Create a new [`LimitReader`] with a [`LimitReader::DEFAULT_BUF_SIZE`] for the limit-readers max threshold.
    #[must_use]
    pub fn new() -> Self {
        Self {
            buf: [0; Self::DEFAULT_BUF_SIZE],
            expected_size: (Self::DEFAULT_BUF_SIZE - 1) as u64,
            decode_zlib: false,
            decode_gzip: false,
        }
    }

    /// Return a reference to the internal buffer.
    #[must_use]
    pub fn buffer(&self) -> &[u8; Self::DEFAULT_BUF_SIZE] {
        &self.buf
    }

    /// Increase the allowed limit on the [`LimitReader`]
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.expected_size = limit;

        self
    }

    /// Enable decoding from compressed Zlib
    pub fn enable_decode_zlib(&mut self) -> &mut Self {
        self.decode_zlib = true;

        self
    }

    #[allow(dead_code)]
    // NOTE: This is private until this is implemented in the future.
    /// Enable decoding from compressed Gzip
    fn enable_decode_gzip(&mut self) -> &mut Self {
        self.decode_gzip = true;

        self
    }

    /// Read from provided source file.  If the source data is already Zlib compressed, optionally decode the data stream before reading it through a limit-reader.
    ///
    /// # Panics
    ///
    /// If the provided source file does not exist or is inaccessible, it will panic.  Refer to [`std::fs::File::open`] for details.  This will return [`LimitReaderError`].
    ///
    /// # Errors
    ///
    /// If this function encounters an error of the kind [`LimitReaderError`], this error will be returned.
    ///
    pub fn read(&mut self, source: PathBuf) -> Result<usize> {
        let f = std::fs::File::open(source).expect("Unable to open file");
        if self.decode_zlib {
            let z = ZlibDecoder::new(f);
            let buf_reader = MyBufReader(z);
            let reader = LimitReaderFallible::new(buf_reader, self.expected_size);

            self.try_read(reader)
        } else {
            let buf_reader = MyBufReader(BufReader::new(f));
            let reader = LimitReaderFallible::new(buf_reader, self.expected_size);

            self.try_read(reader)
        }
    }

    /// Given an accessible source file, this will automatically limit the contents read to the size of the buffer itself.  This will silently truncate read bytes into the buffer, without raising an error.
    ///
    /// # Errors
    ///
    /// If this function encounters an error of the kind [`LimitReaderError`], this error will be returned.
    ///
    pub fn read_limited(&mut self, source: PathBuf) -> Result<LimitReaderOutput> {
        let source_bytes = std::fs::metadata(&source)?.len();
        let f = std::fs::File::open(source)?;

        let bytes_read = if self.decode_zlib {
            let z = ZlibDecoder::new(f);
            let buf_reader = MyBufReader(z);
            let reader = LimitReaderInfallible::new(buf_reader, self.expected_size);

            self.try_read(reader)?
        } else {
            let buf_reader = MyBufReader(BufReader::new(f));
            let reader = LimitReaderInfallible::new(buf_reader, self.expected_size);

            self.try_read(reader)?
        };

        Ok(LimitReaderOutputBuilder::default()
            .source_size(source_bytes)
            .bytes_read(bytes_read as u64)
            .build()?)
    }

    fn try_read(&mut self, mut reader: impl Readable) -> Result<usize> {
        let try_read = reader.perform_read(&mut self.buf);
        match try_read {
            Ok(value) => Ok(value),
            Err(err) => Err(LimitReaderError::new(error::ErrorKind::IoError, err)),
        }
    }
}

/// [`LimitReader`]'s output
#[allow(missing_docs)]
#[derive(Default, Builder)]
#[builder(setter(into))]
pub struct LimitReaderOutput {
    source_size: u64,
    bytes_read: u64,
}

impl LimitReaderOutput {
    /// Return bytes read by the underlying reader.
    #[must_use]
    pub fn bytes_read(&self) -> u64 {
        self.bytes_read
    }

    /// Size in bytes of the underlying file accessible to the reader.
    #[must_use]
    pub fn source_size(&self) -> u64 {
        self.source_size
    }

    /// Unread bytes (from the underlying file accessible to the reader).
    #[must_use]
    pub fn bytes_remaining(&self) -> u64 {
        self.source_size - self.bytes_read
    }
}

impl Display for LimitReaderOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ source_size: {}, bytes_read:{} }}",
            self.source_size, self.bytes_read
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::LimitReader;
    use flate2::write::ZlibEncoder;
    use flate2::Compression;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    mod falible {
        use super::*;

        #[test]
        fn it_works() {
            let dir = tempdir().unwrap();

            let text = "Mike was here. Briefly.";
            let file_path = dir.path().join("test_output.txt");
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "{}", &text).unwrap();

            let mut limit_reader = LimitReader::new();

            match limit_reader.read(file_path.clone()) {
                Ok(read_size) => {
                    let persisted_text =
                        String::from_utf8(limit_reader.buf[..read_size].to_vec()).unwrap();
                    assert_eq!(persisted_text, format!("{}\n", &text).to_string());
                }
                Err(_) => unreachable!(),
            }

            // ZlibDecode
            let mut file = File::create(&file_path).unwrap();
            let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
            e.write_all(text.as_bytes()).unwrap();
            let compressed = e.finish().unwrap();
            file.write_all(&compressed).unwrap();

            let mut limit_reader = LimitReader::new();
            limit_reader.enable_decode_zlib();

            match limit_reader.read(file_path) {
                Ok(read_size) => {
                    let persisted_text =
                        String::from_utf8(limit_reader.buf[..read_size].to_vec()).unwrap();
                    assert_eq!(persisted_text, format!("{}", &text).to_string());
                }
                Err(_) => unreachable!(),
            };

            drop(file);
            dir.close().unwrap();
        }

        #[test]
        fn panic_due_to_limit_constraint() {
            let dir = tempdir().unwrap();

            let text = "Mike was here. Briefly.";
            let file_path = dir.path().join("test_output.txt");
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "{}", &text).unwrap();

            let mut limit_reader = LimitReader::new();
            let limit = 8_u64;
            limit_reader.limit(limit);

            match limit_reader.read(file_path) {
                Ok(read_size) => {
                    assert!(read_size == limit.try_into().unwrap());
                }
                Err(err) => {
                    assert_eq!("Error: too many bytes", err.to_string());
                }
            }

            drop(file);
            dir.close().unwrap();
        }

        #[test]
        fn panic_with_decode_zlib_due_to_limit_constraint() {
            let dir = tempdir().unwrap();

            let text = "Mike was here. Briefly.";
            let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
            e.write_all(text.as_bytes()).unwrap();
            let compressed = e.finish().unwrap();

            let file_path = dir.path().join("test_output.txt");
            let mut file = File::create(&file_path).unwrap();
            file.write_all(&compressed).unwrap();

            let mut limit_reader = LimitReader::new();

            // NOTE: This should error due to exceeding our limit.
            limit_reader.limit(8);

            match limit_reader.read(file_path) {
                Ok(read_size) => {
                    let persisted_text =
                        String::from_utf8(limit_reader.buf[..read_size].to_vec()).unwrap();
                    assert_eq!(persisted_text, format!("{}", &text).to_string());
                }
                Err(err) => assert_eq!("Error: too many bytes", err.to_string()),
            };

            drop(file);
            dir.close().unwrap();
        }

        #[test]
        fn panic_decode_zlib_error_on_corrupt_deflate_stream() {
            let dir = tempdir().unwrap();

            let text = "Mike was here. Briefly.";
            let file_path = dir.path().join("test_output.txt");
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "{}", &text).unwrap();

            let mut limit_reader = LimitReader::new();
            limit_reader.enable_decode_zlib();

            match limit_reader.read(file_path) {
                Ok(_) => unreachable!(),
                Err(err) => assert_eq!("Error: corrupt deflate stream", err.to_string()),
            };

            drop(file);
            dir.close().unwrap();
        }
    }

    mod infalible {
        use super::*;

        #[test]
        fn it_works() {
            let dir = tempdir().unwrap();

            let text = "Mike was here. Briefly.";
            let file_path = dir.path().join("test_output.txt");
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "{}", &text).unwrap();

            let mut limit_reader = LimitReader::new();
            let limit = 8_u64;
            limit_reader.limit(limit);

            match limit_reader.read_limited(file_path.clone()) {
                Ok(reader_output) => {
                    let bytes_read = reader_output.bytes_read();
                    assert!(bytes_read == limit)
                }
                Err(_) => unreachable!(),
            }

            // ZlibDecode
            let mut file = File::create(&file_path).unwrap();
            let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
            e.write_all(text.as_bytes()).unwrap();
            let compressed = e.finish().unwrap();
            file.write_all(&compressed).unwrap();

            let mut limit_reader = LimitReader::new();
            limit_reader.limit(limit).enable_decode_zlib();

            match limit_reader.read_limited(file_path.clone()) {
                Ok(reader_output) => {
                    let bytes_read = reader_output.bytes_read();
                    let persisted_text =
                        String::from_utf8(limit_reader.buf[..(bytes_read as usize)].to_vec())
                            .unwrap();
                    assert_eq!(
                        persisted_text,
                        format!("{}", &text[..(bytes_read as usize)]).to_string()
                    );
                }
                Err(_) => unreachable!(),
            };

            drop(file);
            dir.close().unwrap();
        }

        #[test]
        fn panic_decode_zlib_error_on_corrupt_deflate_stream() {
            let dir = tempdir().unwrap();

            let text = "Mike was here. Briefly.";
            let file_path = dir.path().join("test_output.txt");
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "{}", &text).unwrap();

            let mut limit_reader = LimitReader::new();
            let limit = 8_u64;
            limit_reader
                // RA block
                .limit(limit)
                .enable_decode_zlib();

            match limit_reader.read(file_path) {
                Ok(_) => unreachable!(),
                Err(err) => assert_eq!("Error: corrupt deflate stream", err.to_string()),
            };

            drop(file);
            dir.close().unwrap();
        }
    }
}
