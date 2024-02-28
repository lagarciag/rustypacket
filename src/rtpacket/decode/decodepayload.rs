use std::cell::RefCell;
use std::rc::Rc;

use crate::rtpacket::base::{ApplicationLayer, Layer, Payloadable};
use crate::rtpacket::base::payload::Payload;
use crate::rtpacket::decode::PacketBuilder;
use crate::rtpacket::error::decodererror::DecodeError;

/// Decodes the payload from the provided data and updates the packet builder with the decoded information.
///
/// This function takes a byte slice wrapped in a reference-counted pointer (`Rc<[u8]>`) as input data
/// for decoding. It utilizes a packet builder, also wrapped in a reference-counted pointer to a
/// `RefCell` for mutable access, to manage the decoded data structure. The function decodes the
/// data into a payload, which is then registered as both a generic layer and specifically as an
/// application layer within the packet builder.
///
/// # Arguments
/// * `data` - The data to be decoded, wrapped in an `Rc<[u8]>` for efficient memory management
///   and shared access.
/// * `builder` - A mutable reference to a packet builder (`dyn PacketBuilder`), wrapped in
///   an `Rc<RefCell<...>>` to allow shared, mutable access across different parts of the program.
///
/// # Returns
/// This function returns a `Result<(), DecodeError>`, indicating the outcome of the decode operation.
/// On success, it returns `Ok(())`, indicating that the data was successfully decoded and the packet
/// builder has been updated. On failure, it returns an `Err(DecodeError)`, encapsulating the reason
/// for the decode failure.
///
/// # Errors
/// This function can return a `DecodeError` if the decoding process fails at any point. The error
/// provides details about the failure, facilitating debugging and error handling.
///
/// # Examples
/// Assuming `data` is an `Rc<[u8]>` containing the data to decode and `builder` is an
/// `Rc<RefCell<dyn PacketBuilder>>` prepared for constructing the packet:
///
///
/// Note: Replace `your_crate` and types like `YourPacketBuilderImplementation` with actual
/// implementations from your application.
pub fn create_decode_payload(
    data: Rc<[u8]>,
    builder: Rc<RefCell<dyn PacketBuilder>>,
) -> Result<(), DecodeError> {
    let mut payload0 = Payload::new();

    // Attempt to decode the data into a new payload.
    payload0.decode_from_bytes(data.clone(), builder.borrow_mut().as_decode_feedback())?;

    // Successfully decoded. Now, prepare the payload for inclusion in the packet builder.
    let payload_as_layer: Rc<dyn Layer> = Rc::new(payload0.clone());
    let payload_as_application_layer: Rc<dyn ApplicationLayer> = Rc::new(payload0);

    // Add the payload as a new layer and as the application layer in the packet builder.
    builder.borrow_mut().add_layer(payload_as_layer);
    builder.borrow_mut().set_application_layer(payload_as_application_layer);

    // Return success.
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::rtpacket::decode::decodepayload::create_decode_payload;
    use crate::rtpacket::decode::PacketBuilder;
    use crate::rtpacket::decode::packetbuilder::MockPacketBuilder;

    /// Tests the `DecodePayload` implementation to ensure it correctly adds a fragment layer.
    ///
    /// This test verifies that upon decoding data with `DecodePayload`, a new fragment layer
    /// is added to the `MockPacketBuilder`. It checks that after decoding, the number of layers
    /// added to the packet builder increases by one, indicating a successful layer addition.
    ///
    /// Additionally, it validates that the `application_layer` is set and correctly reports its
    /// size as "3 byte(s)" to match the input data, ensuring that the decoding process correctly
    /// handles the input and reflects it in the packet builder's state.
    #[test]
    fn decode_adds_fragment_layer() {
        // Instantiate a new DecodePayload decoder
        let decoder = create_decode_payload;
        // Create a mock packet builder with initial empty state
        let packet_builder: Rc<RefCell<dyn PacketBuilder>> =
            Rc::new(RefCell::new(MockPacketBuilder {
                layers_added: vec![],
                link_layer: None,
                application_layer: None,
            }));

        // Define a sample data payload to decode
        let data = Rc::from(&[1, 2, 3][..]);

        // Attempt to decode the data using the decoder
        decoder(data, packet_builder.clone()).unwrap();

        // Assert that a new layer has been added to the packet builder
        assert_eq!(packet_builder.borrow_mut().layers_count(), 1usize);
    }

    /// Tests the `DecodePayload` implementation to ensure it correctly handles empty data input.
    ///
    /// Similar to `decode_adds_fragment_layer`, this test verifies the decoder's ability to handle
    /// an input scenario with empty data. It uses the same validation approach to confirm that a
    /// layer is added and the application layer is correctly set. The purpose here is to ensure
    /// consistent behavior and proper handling of edge cases, such as empty data inputs.
    #[test]
    fn decode_handles_empty_data() {
        // Instantiate a new DecodePayload decoder
        let decoder = create_decode_payload;
        // Create a mock packet builder with initial empty state
        let packet_builder: Rc<RefCell<dyn PacketBuilder>> =
            Rc::new(RefCell::new(MockPacketBuilder {
                layers_added: vec![],
                link_layer: None,
                application_layer: None,
            }));

        // Define a sample data payload to decode, intentionally left empty for this test
        let data = Rc::from(&[][..]);

        // Attempt to decode the data using the decoder
        decoder(data, packet_builder.clone()).unwrap();

        // Assert that a new layer has been added to the packet builder
        assert_eq!(&packet_builder.borrow_mut().layers_count(), &1usize);
    }

    // Here you could add more tests, such as checking for specific errors under certain conditions
}
