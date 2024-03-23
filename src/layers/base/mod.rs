use std::cell::RefCell;
use std::fmt::Error;
use std::rc::Rc;

use crate::rtpacket::base::Layer;
use crate::rtpacket::decode::PacketBuilder;
use crate::rtpacket::error::{ErrorDecodeable, PacketError};
use crate::rtpacket::error::decodeerror::DecodeError;
use crate::rtpacket::layertype::LayerType;

/// `BaseLayer` is a convenience struct that holds the data for a layer
/// in a network packet, encapsulating both the contents of the layer itself
/// and the payload it carries.
pub struct BaseLayer {
    /// The bytes that make up this layer. For example, in an Ethernet packet,
    /// this would be the bytes of the Ethernet frame.
    contents: Rc<[u8]>,

    /// The bytes contained by this layer but not part of the layer. For instance,
    /// in Ethernet, this would be the payload of the Ethernet frame.
    payload: Rc<[u8]>,
}

impl BaseLayer {
    /// Creates a new `BaseLayer` with the specified contents and payload.
    ///
    /// # Arguments
    ///
    /// * `contents` - A reference-counted slice of bytes representing the layer's contents.
    /// * `payload` - A reference-counted slice of bytes representing the layer's payload.
    ///
    /// # Returns
    ///
    /// A `BaseLayer` instance.
    pub fn new(contents: Rc<[u8]>, payload: Rc<[u8]>) -> Self {
        Self { contents, payload }
    }

    /// Returns a shared reference to the layer's contents.
    pub fn contents(&self) -> Rc<[u8]> {
        Rc::clone(&self.contents)
    }

    /// Returns a shared reference to the layer's payload.
    pub fn payload(&self) -> Rc<[u8]> {
        Rc::clone(&self.payload)
    }
}


fn decoding_layer_decoder(
    mut decoder: Rc<RefCell<dyn Layer>>,
    data: Rc<[u8]>,
    packet_builder: Rc<RefCell<dyn PacketBuilder>>,
) -> Result<(), DecodeError>

{
    let decode_error = decoder.borrow_mut().decode_from_bytes(data.clone(), packet_builder.borrow_mut().as_decode_feedback());

    match decode_error {
        Ok(_) => {}
        Err(e) => {
            match e {
                PacketError::MethodNotImplemented(e) => {
                    return Err(DecodeError::new("no decoding layer method found", Error::from(e)));
                }
                _ => {
                    return Err(DecodeError::from(e));
                }
            }
        }
    }

    if let Err(e) = decode_error {
        return Err(e);
    }

    packet_builder.add_layer(decoder);

    let next = decoder.next_layer_type();
    if next == LayerType::LayerTypeZero {
        return Ok(());
    }

    packet_builder.next_decoder(next)
}
