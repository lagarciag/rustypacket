use std::cell::RefCell;
use std::rc::Rc;

pub use crate::rtpacket::decode::decodefeedback::DecodeFeedback;
use crate::rtpacket::decode::decodefragment::decode_fragment;
use crate::rtpacket::decode::decodepayload::create_decode_payload;
use crate::rtpacket::decode::decodeunknown::create_decode_unknown;
pub use crate::rtpacket::decode::packetbuilder::PacketBuilder;
use crate::rtpacket::error::decodeerror::DecodeError;
pub use crate::rtpacket::layertype::LayerType;
use crate::rtpacket::layertype::LayerTypes;

pub mod decodefailure;
pub mod decodefeedback;
pub mod decodefragment;
pub mod decodepayload;

pub mod decodeunknown;
pub mod nildecodefeedback;
pub mod packetbuilder;
/// Constructs a decoding function based on the specified layer type.
///
/// This function serves as a factory or builder that, given a layer type, returns the
/// appropriate decoding function for that type. It maps different layer types to their
/// corresponding decoding functions, facilitating dynamic decoding strategy selection
/// based on the layer type being processed.
///
/// # Arguments
/// * `layer_type`: The type of layer for which a decoding function is required. The `layer_type`
///   parameter is used to determine which specific decoding function should be returned.
///
/// # Returns
/// * `DecodeFunc`: A function pointer to the decoding function appropriate for the given
///   layer type. This function can then be called to perform decoding operations on packet data.
///
/// # Supported Layer Types
/// - `LayerTypeZero`: Returns a function for handling unknown or unimplemented layer types.
/// - `LayerTypeDecodeFailure`: Returns a function for handling decoding failures, typically
///   used as a fallback or error handling strategy.
/// - `LayerTypePayload`: Returns a function specifically designed for decoding payload data.
/// - `LayerTypeFragment`: Returns a function for decoding fragmented data, useful for processing
///   packets that are part of a larger set or stream of data fragments.
///
/// ```
pub fn decoder_builder(layer_type: LayerTypes) -> DecodeFunc {
    match layer_type {
        LayerTypes::LayerTypeZero => create_decode_unknown,
        LayerTypes::LayerTypeDecodeFailure => create_decode_unknown,
        LayerTypes::LayerTypePayload => create_decode_payload,
        LayerTypes::LayerTypeFragment => decode_fragment,
    }
}

/// Type alias for a decoding function.
///
/// This type represents a function signature for decoding operations within the packet
/// processing system. A function matching this signature is expected to take packet data
/// and a packet builder as input, perform decoding operations based on the packet's contents,
/// and then use the packet builder to record or act upon the decoding outcome.
///
/// # Parameters
/// * `Rc<[u8]>`: A reference-counted slice of bytes representing the packet data to be decoded.
///   This allows for shared ownership of the data across multiple components without copying.
/// * `Rc<RefCell<dyn PacketBuilder>>`: A reference-counted, mutable reference to a dynamic
///   `PacketBuilder` trait object. This allows for shared, mutable access to a packet builder
///   that can be used to construct or modify a packet based on the decoding results.
///
/// # Returns
/// * `Result<(), Box<dyn Error>>`: The result of the decoding operation. On success, it returns
///   `Ok(())`. On failure, it returns an error boxed as a dynamic `Error` trait object, allowing
///   for various types of errors to be returned.
pub type DecodeFunc = fn(Rc<[u8]>, Rc<RefCell<dyn PacketBuilder>>) -> Result<(), DecodeError>;
