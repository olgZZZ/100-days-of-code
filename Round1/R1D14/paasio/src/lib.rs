use std::io::{Read, Result, Write};

// (::std::marker::PhantomData<R>)
pub struct ReadStats<R> {
    wrapped: R,
    bytes_count: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self { 
            wrapped, 
            bytes_count: 0, 
            reads: 0, 
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_count
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;

        self.wrapped.read(buf).and_then(|bytes| {
            self.bytes_count += bytes;

            Ok(bytes)
        })
    }
}

// (::std::marker::PhantomData<W>)
pub struct WriteStats<W> {
    wrapped: W,
    bytes_count: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self { 
            wrapped, 
            bytes_count: 0, 
            writes: 0
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_count
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;

        self.wrapped.write(buf).and_then(|bytes| {
            self.bytes_count += bytes;

            Ok(bytes)
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}


