use std::cell::RefCell;
use std::rc::Rc;

//use crate::rtpacket::decode::decoder::Decoder;
use crate::rtpacket::decode::PacketBuilder;
use crate::rtpacket::error::decodeerror::DecodeError;
use crate::rtpacket::error::ErrorDecodeable;

/// Attempts to decode a data packet, returning an error if the layer type is not supported.
///
/// # Parameters
///
/// * `_data`: The packet data to be decoded, wrapped in a `Rc<[u8]>` for shared ownership.
/// * `mut _builder`: A mutable, reference-counted `RefCell` wrapping a dynamic `PacketBuilder` trait object.
///
/// # Returns
///
/// Always returns a `Result` type indicating failure with a `DecodeError` encapsulating details of the error.
/// ```
pub fn create_decode_unknown(
    _data: Rc<[u8]>,
    mut _builder: Rc<RefCell<dyn PacketBuilder>>,
) -> Result<(), DecodeError> {
    Err(DecodeError::new("decode unknown layer type", None))
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::rtpacket::decode::packetbuilder::MockPacketBuilder;

    use super::*;

    #[test]
    fn decode_always_fails() {
        let decoder = create_decode_unknown; // Assuming create_decode_unknown is a function that returns a DecodeFunc

        let vec = vec![]; // This Vec<u8> owns its data
        let data: Rc<[u8]> = vec.into(); // Convert Vec<u8> into Rc<[u8]> directly

        let packet_builder: Rc<RefCell<dyn PacketBuilder>> =
            Rc::new(RefCell::new(MockPacketBuilder {
                layers_added: vec![],
                link_layer: None,
                application_layer: None,
            }));

        // Now calling the decoder with mock data and packet_builder
        let result = decoder(data, packet_builder);

        match result {
            Err(e) => {
                // Now this matches the expected type, and you can inspect the error
                println!("{:?}", e);
                // You might want to assert specific error content here
            }
            Ok(_) => assert!(false, "Decoder should not succeed"),
        }

        println!("test passed");
    }
}
