use std::error::Error;
use std::rc::Rc;

// Assumed external crate for lazy_static
use crate::rtpacket::checksum::ChecksumVerificationResult;
use crate::rtpacket::decode::{DecodeFeedback, LayerType};
use crate::rtpacket::error::decodererror::DecodeError;
use crate::rtpacket::layerclass::LayerClass;

pub(crate) mod fragment;
pub(crate) mod payload;

// Common trait for all layers, providing basic methods.
pub trait Layer {
    // Returns the LayerType of this layer.
    fn layer_type(&self) -> LayerType;

    // Returns the contents of this layer.
    fn layer_contents(&self) -> Option<Rc<[u8]>>;

    // Returns the payload within this layer.
    fn layer_payload(&self) -> Option<Rc<[u8]>>;
    fn string(&self) -> String;
}

/// Trait for layers that contain a checksum which can be verified after
/// a packet has been decoded.
pub trait LayerWithChecksum {
    /// Verifies the checksum and returns the result as a `Result` type,
    /// encapsulating `ChecksumVerificationResult` on success, or an error message on failure.
    fn verify_checksum(&self) -> Result<ChecksumVerificationResult, Box<dyn Error>>;
}

// Trait for layers that contain a payload.
pub trait Payloadable: Layer {
    // Returns the payload of this layer.
    fn can_decode(&self) -> impl LayerClass;
    fn next_layer_type(&self) -> LayerType;
    //fn decode_from_bytes(&mut self, data: &[u8], df: &mut impl DecodeFeedback) -> Result<(), Box<dyn std::error::Error>>;
    fn decode_from_bytes(
        &mut self,
        data: Rc<[u8]>,
        _df: Box<dyn DecodeFeedback>,
    ) -> Result<(), DecodeError>;
}

/// Common trait for layers that have a flow, such as Link, Network, and Transport layers.
pub trait Flow {
    // Assuming a method to get flow information; specifics depend on the application.
    fn flow(&self) -> String;
}

/// Trait for the Link Layer (Layer 2 in OSI, Layer 1 in TCP/IP).
pub trait LinkLayer: Layer {
    fn link_flow(&self) -> String;
}

/// Trait for the Network Layer (Layer 3 in OSI, Layer 2 in TCP/IP).
pub trait NetworkLayer: Layer {
    fn network_flow(&self) -> String;
}

/// Trait for the Transport Layer (Layer 4 in OSI, Layer 3 in TCP/IP).
pub trait TransportLayer: Layer {
    fn transport_flow(&self) -> String;
}

/// Trait for the Application Layer (Layer 7 in OSI, Layer 4 in TCP/IP),
/// which is also known as the packet payload.
pub trait ApplicationLayer: Layer {
    fn payload(&self) -> Option<Rc<[u8]>>;
}

/// Represents a packet layer created when decoding of the packet has failed.
///
/// Its payload is all the bytes that we were unable to decode, and the returned
/// error details why the decoding failed.
pub trait ErrorLayer: Layer {
    fn error(&self) -> &(dyn Error + 'static);
}
