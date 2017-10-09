/// `blocking` turns a non-blocking transmit/receive into a blocking transmit/receive
pub fn blocking<F, O, E>(non_blocking: F) -> Result<O, E>
    where F: Fn() -> Result<O, E>,
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

/// Common transmit/receive errors
/// This list is intended to grow over time and it is not recommended to exhaustively match against it
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Error {
    
    /// In case of transmissions: Buffer full. In case of reception: Buffer empty
    BufferExhausted,
    InvalidInput,
    Other,
}
