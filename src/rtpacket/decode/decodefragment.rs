use std::cell::RefCell;
use std::rc::Rc;

use crate::rtpacket::base::{ApplicationLayer, Layer, Payloadable};
use crate::rtpacket::base::fragment::Fragment;
use crate::rtpacket::decode::{DecodeFunc, PacketBuilder};
use crate::rtpacket::error::decodererror::{DecodeError, ErrorDecodeable};

pub fn fragment_decoder() -> DecodeFunc {
    decode_fragment
}

/// Decodes a fragment from the provided data and updates the packet builder with the decoded fragment.
///
/// This function attempts to decode a given fragment using the provided data. Upon successful
/// decoding, it adds the decoded fragment as a new layer to the packet builder. If applicable,
/// it also sets the decoded fragment as the application layer. If the decoding fails, it returns
/// a `DecodeError` encapsulating the failure reason.
///
/// # Arguments
/// * `data` - The data to be decoded, wrapped in a `Rc<[u8]>` for shared ownership among multiple
///   parts of the program. This allows efficient data management without unnecessary copies.
/// * `builder` - A reference to a packet builder, wrapped in `Rc<RefCell<dyn PacketBuilder>>`. This
///   allows for shared, mutable access to the packet builder, enabling modifications (like adding
///   layers) based on the decoding results.
///
/// # Returns
/// * `Result<(), DecodeError>` - On success, returns `Ok(())` indicating the fragment was decoded
///   and processed successfully. On failure, returns `Err(DecodeError)` with details about the
///   decoding failure, including an optional source error if available.
///
/// # Examples
/// ```
/// // Assuming the existence of `data` as Rc<[u8]> and `builder` as Rc<RefCell<dyn PacketBuilder>>
/// match decode_fragment(data, builder) {
///     Ok(_) => println!("Fragment decoded successfully."),
///     Err(e) => println!("Failed to decode fragment: {}", e),
/// }
/// ```
pub fn decode_fragment(
    data: Rc<[u8]>,
    builder: Rc<RefCell<dyn PacketBuilder>>,
) -> Result<(), DecodeError> {
    let mut payload0 = Fragment::new();

    payload0.decode_from_bytes(data.clone(), builder.borrow_mut().as_decode_feedback())?;
    let payload_as_layer: Rc<dyn Layer> = Rc::new(payload0.clone());
    let payload_as_application_layer: Rc<dyn ApplicationLayer> = Rc::new(payload0);
    builder.borrow_mut().add_layer(payload_as_layer); // Now passing Rc<dyn Layer>
    builder
        .borrow_mut()
        .set_application_layer(payload_as_application_layer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::rtpacket::decode::decodefragment::decode_fragment;
    use crate::rtpacket::decode::PacketBuilder;
    use crate::rtpacket::decode::packetbuilder::MockPacketBuilder;

    //// Tests the `DecodeFragment`'s ability to add a layer to the `MockPacketBuilder`
    /// and correctly set the application layer based on provided data.
    ///
    /// This test verifies two primary functionalities:
    /// 1. That a new layer is indeed added to the `MockPacketBuilder`.
    /// 2. The application layer is correctly identified and its content matches the expected "3 byte(s)"
    ///    string representation, indicating that the fragment's data size is accurately reported.
    #[test]
    fn decode_adds_fragment_layer() {
        let decoder = decode_fragment;

        let packet_builder: Rc<RefCell<dyn PacketBuilder>> =
            Rc::new(RefCell::new(MockPacketBuilder {
                layers_added: vec![],
                link_layer: None,
                application_layer: None,
            }));

        // Define sample data to decode.
        let vec = vec![1, 2, 3, 4, 5]; // This Vec<u8> owns its data
        let data0: Rc<[u8]> = vec.into(); // Convert Vec<u8> into Rc<[u8]> directly

        decoder(data0, packet_builder.clone()).unwrap();

        // Assert that one layer was indeed added.
        assert_eq!(
            &packet_builder.borrow_mut().layers_count(),
            &1usize,
            "One layer should have been added."
        );
    }
}
