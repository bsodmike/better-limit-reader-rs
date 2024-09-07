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

pub(crate) mod falible {
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
        limit: usize,
        reader_count: usize,
    }

    impl<R> LimitReaderFallible<R>
    where
        R: Read,
    {
        pub fn new(r: R, limit: usize) -> Self {
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
        fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
            // NOTE: using +1 in the range below trips the error.
            buf = &mut buf[..self.limit + 1];

            let bytes_read = self.reader.read(buf)?;
            if bytes_read > self.limit {
                return Err(io::Error::new(io::ErrorKind::Other, "too many bytes"));
            }
            self.limit -= bytes_read;
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
        limit: usize,
        reader_count: usize,
    }

    impl<R> LimitReaderInfallible<R>
    where
        R: Read,
    {
        pub fn new(r: R, limit: usize) -> Self {
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
            let max_read = self.limit.min(buf.len()); // min of limit and buf.len()

            let bytes_read = self.reader.read(&mut buf[..max_read])?;
            self.reader_count += 1;

            Ok(bytes_read)
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
