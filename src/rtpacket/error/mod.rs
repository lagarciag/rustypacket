use std::backtrace::Backtrace;
use std::error::Error;
use std::fmt;
use std::fmt::Display;

use crate::rtpacket::error::decodeerror::DecodeError;
use crate::rtpacket::error::nomethoderror::MethodNotImplementedError;
use crate::rtpacket::error::verifychecksumerror::VerifyChecksumError;

pub mod decodeerror;
pub mod nomethoderror;
pub mod verifychecksumerror;

pub trait Backtraceable {
    fn backtrace(&self) -> &Backtrace;
}

/// `ErrorDecodeable` is a trait extending `std::error::Error` and `std::fmt::Display`
/// to include functionality specific to decoding errors.
///
/// This trait is designed to be implemented by error types that require a message and
/// potentially a backtrace to aid in debugging decoding operations.
pub trait ErrorDecodeable: Error + Display {
    /// Creates a new instance of an error type implementing `ErrorDecodeable`.
    ///
    /// # Arguments
    ///
    /// * `message` - A descriptive message about the error that occurred.
    ///
    /// # Returns
    ///
    /// A new instance of the error type.
    fn new(message: &str, source: Option<Box<dyn Error>>) -> Self;

    /// Retrieves the error message associated with this error.
    ///
    /// # Returns
    ///
    /// A reference to the error message string.
    fn message(&self) -> &str;
}

#[derive(Debug)]
pub enum PacketError {
    Decode(DecodeError),
    MethodNotImplemented(MethodNotImplementedError),
    VerifyChecksum(VerifyChecksumError),
}
impl From<DecodeError> for PacketError {
    fn from(error: DecodeError) -> Self {
        PacketError::Decode(error)
    }
}

impl From<VerifyChecksumError> for PacketError {
    fn from(error: VerifyChecksumError) -> Self {
        PacketError::VerifyChecksum(error)
    }
}

impl From<MethodNotImplementedError> for PacketError {
    fn from(error: MethodNotImplementedError) -> Self {
        PacketError::MethodNotImplemented(error)
    }
}

impl fmt::Display for PacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PacketError::Decode(e) => write!(f, "DecodeError: {}", e),
            PacketError::MethodNotImplemented(e) => {
                write!(f, "MethodNotImplementedError: {}", e)
            }
            PacketError::VerifyChecksum(e) => write!(f, "VerifyChecksumError: {}", e),
        }
    }
}

impl Error for PacketError {}
