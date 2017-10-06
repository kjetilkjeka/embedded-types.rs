/// `blocking_transmit` turns a non-blocking transmit into a blocking transmit
pub fn blocking_transmit<F, O>(transmit: F) -> Result<O, TransmitError>
    where F: Fn() -> Result<O, TransmitError> {
    loop {
        match transmit() {
            Err(TransmitError::BufferFull) => (),
            x => return x,
        }            
    }
}

/// `blocking_receive` turns a non-blocking receive into a blocking receive
pub fn blocking_receive<F, O>(receive: F) -> Result<O, ReceiveError>
    where F: Fn() -> Result<O, ReceiveError> {
    loop {
        match receive() {
            Err(ReceiveError::BufferEmpty) => (),
            x => return x,
        }            
    }
}

/// Common transmit errors.
/// This list is intended to grow over time and it is not recommended to exhaustively match against it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransmitError {
    BufferFull,
}

/// Common receive errors.
/// This list is intended to grow over time and it is not recommended to exhaustively match against it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReceiveError {
    BufferEmpty,
}
