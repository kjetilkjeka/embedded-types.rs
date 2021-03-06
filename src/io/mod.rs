//! Traits, helpers and type definitions for embedded/no_std I/O functionality.

use core::fmt;
use core::result;

/// `blocking` turns a non-blocking transmit/receive into a blocking transmit/receive
pub fn blocking<F, O, E>(non_blocking: F) -> result::Result<O, E>
    where F: Fn() -> result::Result<O, E>,
          E: Into<Error> + Clone {
    loop {
        match non_blocking() {
            Err(x) => {
                if x.clone().into() != Error::BufferExhausted {
                    return Err(x);
                }
            },
            Ok(x) => {
                return Ok(x);
            },
        }            
    }
}

/// A specialized `Result` type for embedded I/O operations.
pub type Result<T> = result::Result<T, Error>;

/// Common transmit/receive errors.
/// This list is intended to grow over time and it is not recommended to exhaustively match against it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    
    /// In case of transmissions: Buffer full. In case of reception: Buffer empty.
    BufferExhausted,
    InvalidInput,

    /// A reception can fail with this error if it's grounded in the parity checking, CRC calculation or similar.
    ErrorDetectionCode,
    
    Other,
}

/// A trait for objects which are byte-oriented sinks.
///
/// This is very similar to the standard library's `io::Write` and share similiarities with `fmt::Write`.
/// This trait is intended to be implemented for custom types used in no_std development.
pub trait Write {
    /// Write a buffer into this object, returning how many bytes were written.
    ///
    ///This function will attempt to write the entire contents of buf, but the entire write may not succeed, or the write may also generate an error. A call to write represents at most one attempt to write to any wrapped object.
    fn write(&mut self, buf: &[u8]) -> Result<usize>;

    /// Attempts to write an entire buffer into this write.
    ///
    /// This method will continously call write untill there is no more data or an error of non `Error::BufferExhausted` kind is returned.
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        let mut bytes_written = 0;
        while bytes_written < buf.len() {
            match self.write(&buf[bytes_written..]) {
                Ok(n) => bytes_written += n,
                Err(Error::BufferExhausted) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
    
    /// Attempts to write a str into this write.
    ///
    /// This method will continously call write untill there is no more data or an error of non `Error::BufferExhausted` kind is returned.
    fn write_str(&mut self, s: &str) -> Result<()> {
        self.write_all(s.as_bytes())
    }

    /// Writes a formatted string into this writer, returning any error encountered.
    ///
    /// This method will continously call write untill there is no more data or an error of non `Error::BufferExhausted` kind is returned.
    #[allow(unused_must_use)]
    fn write_fmt(&mut self, args: fmt::Arguments) -> Result<()> {
        // This Adapter is needed to allow `self` (of type `&mut
        // Self`) to be cast to a Write (below) without
        // requiring a `Sized` bound.
        struct Adapter<'a, T>
            where T: ?Sized + 'a {
            inner: &'a mut T,
            error: Option<Error>,
        };
        
        impl<'a, T> fmt::Write for Adapter<'a, T>
            where T: Write + ?Sized {
            fn write_str(&mut self, s: &str) -> result::Result<(), fmt::Error> {
                match Write::write_str(self.inner, s) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Some(e);
                        Err(fmt::Error)
                    },
                }
            }
        }

        let mut adapter = Adapter{
            inner: self,
            error: None,
        };

        fmt::Write::write_fmt(&mut adapter, args);
        
        match adapter.error {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

/// The `Read` trait allows for reading bytes from a source.
///
/// Implementors of the `Read` trait are called 'readers'.
///
/// This is very similar to the standard library's `io::Read`.
/// This trait is intended to be implemented for custom types used in no_std development.
pub trait Read {
    /// This function will read bytes from the underlying stream until the buffer is full, delimiter is found or EOF is found.
    ///
    /// If the return value of this method is `Ok(n)` then it must be guaranteed that `0 >= n <= buf.len()`.
    /// A nonzero `n` value indicates that the buffer `buf` has been fieeld in with `n` bytes of data from this source.
    /// If `n` is 0, then it can indicate one of two scenarios:
    ///
    /// 1. This reader has reached its "end of file" and will likely no longer be able to produce bytes. Note that this does not mean that the reader will always no longer be able to produce bytes.
    /// 2. The buffer specified was 0 bytes in length.
    ///
    /// No guarantees are provided about the contents of buf when this function is called, implementations cannot rely on any property of the contents of buf being true.
    /// It is recommended that implementations only write data to buf instead of reading its contents.
    fn read_until(&mut self, byte: u8, buf: &mut [u8]) -> Result<usize>;
}

#[cfg(test)]
mod tests {

    use io::*;
    
    #[test]
    fn write_test() {
        struct TestBuffer {
            buffer: [u8; 100],
            index: usize,
        }
        
        impl Write for TestBuffer {
            fn write(&mut self, buf: &[u8]) -> Result<usize> {
                self.buffer[self.index..self.index+buf.len()].clone_from_slice(buf);
                self.index += buf.len();
                Ok(buf.len())
            }
        }

        let mut test_buffer = TestBuffer{buffer: [0u8; 100], index: 0};
        
        write!(test_buffer, "This {} a {}", "is", "test").unwrap();

        assert_eq!(test_buffer.buffer[..test_buffer.index].len(), "This is a test".as_bytes().len());
        assert_eq!(&test_buffer.buffer[..test_buffer.index], "This is a test".as_bytes());
    }
}
