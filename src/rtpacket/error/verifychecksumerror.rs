use std::backtrace::Backtrace;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::rtpacket::error::{Backtraceable, ErrorDecodeable};

/// `DecodeError` is a custom error type designed for use in decoding operations.
/// It contains an error message and a stack trace to aid in diagnosing issues.
#[derive(Debug)]
pub struct VerifyChecksumError {
    pub message: String,
    pub stack_trace: Backtrace,
    pub source: Option<Box<dyn Error>>,
}

impl Display for VerifyChecksumError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}\nStack trace:\n{:?}", self.message, self.stack_trace)
    }
}

impl Error for VerifyChecksumError {
    // Override the source method to return the cause of the error
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

impl ErrorDecodeable for VerifyChecksumError {
    /// Constructs a new `DecodeError` with the specified message and captures the current stack trace.
    fn new(message: &str, source: Option<Box<dyn Error>>) -> Self {
        VerifyChecksumError {
            message: message.to_string(),
            stack_trace: Backtrace::capture(),
            source,
        }
    }

    /// Returns the error message.
    fn message(&self) -> &str {
        &self.message
    }
}

impl Backtraceable for VerifyChecksumError {
    fn backtrace(&self) -> &Backtrace {
        &self.stack_trace
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    // Test the creation of a DecodeError and retrieving its message
    #[test]
    fn decode_error_new_and_message() {
        let msg = "test error message";
        let error = VerifyChecksumError::new(msg, None);

        assert_eq!(
            error.message(),
            msg,
            "The error message should match the input message."
        );
    }

    // Test the Display trait implementation for DecodeError
    #[test]
    fn decode_error_display() {
        let msg = "display error message";
        let error = VerifyChecksumError::new(msg, None);
        let error_string = format!("{}", error);

        print!("{}", error_string);

        assert!(
            error_string.contains(msg),
            "The Display output should contain the error message."
        );
        assert!(
            error_string.contains("Stack trace:"),
            "The Display output should contain 'Stack trace:'."
        );
    }

    // Test the Error trait implementation for DecodeError
    #[test]
    fn decode_error_trait_impl() {
        let msg = "trait impl error message";
        let error: Box<dyn Error> = Box::new(VerifyChecksumError::new(msg, None));

        // Attempt to downcast to `Backtraceable` to access the backtrace.
        if let Some(backtraceable) = error.downcast_ref::<VerifyChecksumError>() {
            assert_eq!(
                error.to_string(),
                format!("{}\nStack trace:\n{:?}", msg, backtraceable.backtrace()),
                "The error should be correctly converted to a string."
            );
        } else {
            panic!("Failed to downcast to Backtraceable");
        }
    }
}
