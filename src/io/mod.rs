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
type Result<T> = result::Result<T, Error>;

/// Common transmit/receive errors
/// This list is intended to grow over time and it is not recommended to exhaustively match against it
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    
    /// In case of transmissions: Buffer full. In case of reception: Buffer empty
    BufferExhausted,
    InvalidInput,
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

    fn write_str(&mut self, s: &str) -> Result<()> {
        let num_bytes = self.write(s.as_bytes())?;

        if num_bytes < s.len() {
            Err(Error::BufferExhausted)
        } else {
            Ok(())
        }
    }
    
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
