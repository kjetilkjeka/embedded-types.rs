/// `blocking_transmit` turns a non-blocking transmit into a blocking transmit
pub fn blocking_transmit<F, O, E>(transmit: F) -> Result<O, E>
    where F: Fn() -> Result<O, E>,
          E: Into<TransmitError> + Clone {
    loop {
        match transmit() {
            Err(x) => {
                if x.clone().into() != TransmitError::BufferFull {
                    return Err(x);
                }
            },
            Ok(x) => {
                return Ok(x);
            },
        }            
    }
}

/// `blocking_receive` turns a non-blocking receive into a blocking receive
pub fn blocking_receive<F, O, E>(receive: F) -> Result<O, E>
    where F: Fn() -> Result<O, E>,
          E: Into<ReceiveError> + Clone {
    loop {
        match receive() {
            Err(x) => {
                if x.clone().into() != ReceiveError::BufferEmpty {
                    return Err(x);
                }
            },
            Ok(x) => {
                return Ok(x);
            },
        }                        
    }
}

/// Common transmit errors.
/// This list is intended to grow over time and it is not recommended to exhaustively match against it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransmitError {
    BufferFull,
    InvalidInput,
    Other,
}

/// Common receive errors.
/// This list is intended to grow over time and it is not recommended to exhaustively match against it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReceiveError {
    BufferEmpty,
    Other,
}
