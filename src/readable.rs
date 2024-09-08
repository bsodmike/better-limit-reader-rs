#[allow(clippy::wildcard_imports)]
use super::*;

pub struct MyBufReader<Z: Read>(pub Z);

impl<Z: Read> Read for MyBufReader<Z> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

pub trait Readable {
    fn perform_read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}

#[allow(dead_code)]
type ReaderResult<T> = std::result::Result<T, LimitReaderError>;

pub(crate) mod falible {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    impl<R> Readable for LimitReaderFallible<R>
    where
        R: Read,
    {
        fn perform_read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.read(buf)
        }
    }

    pub(crate) struct LimitReaderFallible<R>
    where
        R: Read,
    {
        reader: R,
        limit: u64,
        reader_count: usize,
    }

    impl<R> LimitReaderFallible<R>
    where
        R: Read,
    {
        pub fn new(r: R, limit: u64) -> Self {
            Self {
                reader: r,
                limit,
                reader_count: 0,
            }
        }
    }

    impl<R> Read for LimitReaderFallible<R>
    where
        R: Read,
    {
        #[allow(clippy::cast_possible_truncation)]
        fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
            // NOTE: using +1 in the range below trips the error.
            buf = &mut buf[..=(self.limit as usize)];

            let bytes_read = self.reader.read(buf)?;
            if bytes_read > self.limit as usize {
                return Err(io::Error::new(io::ErrorKind::Other, "too many bytes"));
            }
            self.limit -= bytes_read as u64;
            self.reader_count += 1;

            Ok(bytes_read)
        }
    }

    impl<R> BufRead for LimitReaderFallible<R>
    where
        R: Read,
    {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            unimplemented!("LimitReaderFallible should never call `fill_buf`")
        }

        fn consume(&mut self, _: usize) {}
    }
}

pub(crate) mod infalible {

    #[allow(clippy::wildcard_imports)]
    use super::*;

    impl<R> Readable for LimitReaderInfallible<R>
    where
        R: Read,
    {
        fn perform_read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.read(buf)
        }
    }

    pub(crate) struct LimitReaderInfallible<R>
    where
        R: Read,
    {
        reader: R,
        limit: u64,
        reader_count: usize,
    }

    impl<R> LimitReaderInfallible<R>
    where
        R: Read,
    {
        pub fn new(r: R, limit: u64) -> Self {
            Self {
                reader: r,
                limit,
                reader_count: 0,
            }
        }
    }

    impl<R> Read for LimitReaderInfallible<R>
    where
        R: Read,
    {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            match TryInto::<u64>::try_into(buf.len()) {
                Ok(buf_len) => {
                    let max_read = self.limit.min(buf_len); // min of limit and buf.len()

                    match TryInto::<usize>::try_into(max_read) {
                        Ok(m) => {
                            let bytes_read = self.reader.read(&mut buf[..m])?;
                            self.reader_count += 1;

                            Ok(bytes_read)
                        }
                        Err(_) => Ok(0),
                    }
                }
                Err(_) => Ok(0),
            }
        }
    }

    impl<R> BufRead for LimitReaderInfallible<R>
    where
        R: Read,
    {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            unreachable!("LimitReaderInfallible should never call `fill_buf`")
        }

        fn consume(&mut self, _amt: usize) {}
    }
}
