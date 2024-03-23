use std::rc::Rc;

use crate::rtpacket::base::{ErrorLayer, Layer};
use crate::rtpacket::checksum::ChecksumVerificationResult;
use crate::rtpacket::decode::decoder_builder;
use crate::rtpacket::error::decodeerror::DecodeError;
use crate::rtpacket::error::ErrorDecodeable;
use crate::rtpacket::error::nomethoderror::MethodNotImplementedError;
use crate::rtpacket::error::PacketError;
use crate::rtpacket::layertype::{LayerType, LayerTypeID};
use crate::rtpacket::layertype::LayerTypes::LayerTypeDecodeFailure;

/// Represents a packet layer created when decoding of the packet data fails.
///
/// This struct implements `ErrorLayer`. `LayerContents` will be the entire set
/// of bytes that failed to parse, and `Error` will return the reason parsing
/// failed.
#[derive(Clone)]
pub struct DecodeFailure {
    pub layer_type: Option<LayerType>,
    pub in_data: Option<Rc<[u8]>>,
    pub err: Rc<DecodeError>,
    pub stack: Vec<u8>,
}

impl DecodeFailure {
    /// Creates a new `DecodeFailure` instance.
    ///
    /// # Arguments
    ///
    /// * `data` - A vector of bytes representing the data that was being decoded when the failure occurred.
    /// * `err` - The error that occurred during decoding, encapsulated in a `Box<dyn Error>` to allow for any kind of error.
    /// * `stack` - A vector of bytes representing additional context or the "stack" relevant to the decoding failure, which might be useful for debugging.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `DecodeFailure`.
    pub fn new(data: Rc<[u8]>, err: DecodeError, stack: Vec<u8>) -> Self {
        DecodeFailure {
            layer_type: Some(LayerType {
                id: LayerTypeDecodeFailure as LayerTypeID,
                name: "DecodeFailure".to_owned(),
                decoder: decoder_builder(LayerTypeDecodeFailure),
            }),
            in_data: Some(data),
            err: Rc::new(err),
            stack,
        }
    }

    /// Converts the stack bytes to a UTF-8 string for debugging purposes.
    ///
    /// This method attempts to interpret the `stack` field's bytes as a UTF-8 encoded string and returns it.
    /// If the `stack` contains invalid UTF-8 sequences, they are replaced with the Unicode replacement character (�).
    ///
    /// # Returns
    ///
    /// A `String` containing the UTF-8 decoded bytes from the `stack`. If the `stack` contains invalid UTF-8,
    /// non-UTF-8 bytes are replaced with �.
    pub fn dump(&self) -> String {
        // Directly convert the bytes in `stack` to a String, assuming UTF-8 encoding.
        // This can fail if `stack` contains invalid UTF-8.
        // If handling non-UTF-8 or potentially invalid data, consider using lossy conversion
        // or handling the error more explicitly.
        String::from_utf8_lossy(&self.stack).into_owned()
    }
}

impl ErrorLayer for DecodeFailure {
    fn error(&self) -> DecodeError {
        todo!()
    }
}

impl Layer for DecodeFailure {
    /// Returns the type of the layer.
    ///
    /// This method identifies the `DecodeFailure` layer type, helping users
    /// distinguish it from other possible layer types in the context of packet decoding.
    ///
    /// # Returns
    ///
    /// Returns a constant representing the decode failure layer type.
    fn layer_type(&self) -> LayerType {
        self.layer_type.clone().unwrap()
    }

    /// Provides access to the contents of the layer.
    ///
    /// In the case of a decode failure, the contents are the bytes that were
    /// being decoded when the failure occurred. This can be useful for debugging
    /// or logging purposes.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing a reference to the layer's data as a slice of bytes.
    /// Since `DecodeFailure` always contains data, this method returns `Some`.
    fn layer_contents(&self) -> Option<Rc<[u8]>> {
        self.in_data.clone()
    }

    /// Indicates that this layer type does not have a payload.
    ///
    /// For `DecodeFailure`, the concept of a payload separate from the layer's
    /// main data does not apply. Therefore, this method always returns `None`.
    ///
    /// # Returns
    ///
    /// Returns `None`, indicating no separate payload data for this layer type.
    fn layer_payload(&self) -> Option<Rc<[u8]>> {
        None
    }

    fn verify_checksum(&self) -> Result<ChecksumVerificationResult, PacketError> {
        Err(PacketError::from(MethodNotImplementedError::new(
            "layer does not verify checksum",
            None,
        )))
    }

    /// Provides a descriptive string for the layer.
    ///
    /// This method returns a string that includes the type of the layer and
    /// a description or summary of the error that caused the decode failure.
    /// It's intended for logging or displaying error information in a human-readable format.
    ///
    /// # Returns
    ///
    /// Returns a `String` containing a description of the decode failure, including
    /// the error message.
    fn string(&self) -> String {
        let error_message = format!("DecodeFailure: {:?}", self.err);
        error_message
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::rtpacket::decode::decodefailure::DecodeFailure;
    use crate::rtpacket::error::decodeerror::DecodeError;
    use crate::rtpacket::error::ErrorDecodeable;

    /// Tests that a `DecodeFailure` instance correctly retains and exposes an error message.
    ///
    /// This test verifies that a `DecodeFailure` instance, when initialized with a `DecodeError`,
    /// can downcast the contained `dyn Error` back to a specific `DecodeError` type, and that
    /// the original error message is preserved and accessible.
    #[test]
    fn test_decode_failure_string() {
        let data = Rc::from([1, 2, 3]);
        let err = DecodeError::new("Test error", None);
        let stack = vec![4, 5, 6];
        let decode_failure = DecodeFailure::new(data, err, stack);

        let decode_error = decode_failure.err;
        assert_eq!(decode_error.message(), "Error message");
    }

    /// Tests that `DecodeFailure::dump` correctly handles valid UTF-8 byte sequences.
    ///
    /// Ensures that when `DecodeFailure::dump` is called with a stack containing a valid UTF-8 string,
    /// the returned string matches the expected UTF-8 content exactly.
    #[test]
    fn test_dump_valid_utf8() {
        let data = Rc::from([1, 2, 3]);
        let err = DecodeError::new("Test error", None);
        let stack = "Valid UTF-8 string".as_bytes().to_vec();
        let decode_failure = DecodeFailure::new(data, err, stack);

        let dumped_string = decode_failure.dump();
        assert_eq!(dumped_string, "Valid UTF-8 string");
    }

    /// Tests that `DecodeFailure::dump` correctly processes invalid UTF-8 byte sequences.
    ///
    /// This test verifies that when `DecodeFailure::dump` encounters invalid UTF-8 byte sequences
    /// within the `stack`, it replaces those invalid sequences with the Unicode replacement character (�),
    /// ensuring that the method always returns a valid UTF-8 `String`.
    #[test]
    fn test_dump_invalid_utf8() {
        let data = Rc::from([1, 2, 3]);
        let err = DecodeError::new("Test error", None);
        let stack = vec![0xff, 0xfe, 0xfd];
        let decode_failure = DecodeFailure::new(data, err, stack);

        let dumped_string = decode_failure.dump();
        assert_eq!(dumped_string, "\u{FFFD}\u{FFFD}\u{FFFD}");
    }
}
