
/// Common transmit errors.
/// This list is intended to grow over time and it is not recommended to exhaustively match against it.
pub enum TransmitError {
    BufferFull,
}

/// Common receive errors.
/// This list is intended to grow over time and it is not recommended to exhaustively match against it.
pub enum ReceiveError {
    BufferEmpty,
}
