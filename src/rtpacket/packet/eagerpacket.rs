use std::cell::RefCell;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Write;
use std::rc::Rc;
use std::time::SystemTime;

use crate::rtpacket::base::{
    ApplicationLayer, ErrorLayer, Layer, LayerWithChecksum, LinkLayer, NetworkLayer, TransportLayer,
};
use crate::rtpacket::capture::PacketMetadata;
use crate::rtpacket::checksum::ChecksumMismatch;
use crate::rtpacket::decode::{DecodeFeedback, DecodeFunc, PacketBuilder};
use crate::rtpacket::decode::decodefailure::DecodeFailure;
use crate::rtpacket::error::decodeerror::{DecodeError, NoLastLayerError};
use crate::rtpacket::error::ErrorDecodeable;
use crate::rtpacket::error::PacketError;
use crate::rtpacket::error::verifychecksumerror::VerifyChecksumError;
use crate::rtpacket::layerclass::LayerClass;
use crate::rtpacket::layertype::LayerTypeID;
use crate::rtpacket::packet::{DecodeOptions, layer_dump};
use crate::rtpacket::packet::Packetable;

/// Converts a fixed-size array into an `Rc<[u8]>`.
///
/// This function takes a fixed-size array of bytes and converts it into a reference-counted
/// dynamically sized slice (`Rc<[u8]>`). This conversion involves moving the array to the heap
/// and then wrapping it with `Rc` to allow shared ownership.
///
/// # Arguments
///
/// * `array` - A fixed-size array of bytes (`[u8; N]`).
///
/// # Returns
///
/// A `Rc<[u8]>` pointing to the heap-allocated slice.
fn convert_array_to_rc_slice<const N: usize>(array: [u8; N]) -> Rc<[u8]> {
    let boxed_slice: Box<[u8]> = array.into();
    Rc::from(boxed_slice)
}

//create_packet_struct!(EagerPacket);
#[derive(Clone)]
pub struct EagerPacket {
    /// Contains the entire packet data.
    pub data: Rc<[u8]>,
    /// Space for an initial set of layers already created inside the packet.
    pub initial_layers: Vec<Rc<dyn Layer>>,
    /// Contains each layer that has been decoded.
    pub layers: Vec<Rc<dyn Layer>>,
    /// The last layer added to the packet.
    pub last: Option<Rc<dyn Layer>>,
    /// Metadata associated with this packet.
    pub metadata: PacketMetadata,
    /// Decoding options for this packet.
    pub decode_options: DecodeOptions,

    /// Pointer to the various important layers.
    pub link: Option<Rc<dyn LinkLayer>>,
    pub network: Option<Rc<dyn NetworkLayer>>,
    pub transport: Option<Rc<dyn TransportLayer>>,
    pub application: Option<Rc<dyn ApplicationLayer>>,
    pub failure: Option<Rc<dyn ErrorLayer>>,
}

impl DecodeFeedback for EagerPacket {
    fn set_truncated(&mut self) {
        self.metadata.truncated = true;
    }
}

impl PacketBuilder for EagerPacket {
    fn add_layer(&mut self, layer: Rc<dyn Layer>) {
        self.layers.push(layer.clone());
        self.last = Some(layer);
    }

    fn set_link_layer(&mut self, l: Rc<dyn LinkLayer>) {
        if self.link.is_none() {
            self.link = Some(l);
        }
    }

    fn set_network_layer(&mut self, l: Rc<dyn NetworkLayer>) {
        if self.network.is_none() {
            self.network = Some(l);
        }
    }

    fn set_transport_layer(&mut self, l: Rc<dyn TransportLayer>) {
        if self.transport.is_none() {
            self.transport = Some(l);
        }
    }

    fn set_application_layer(&mut self, l: Rc<dyn ApplicationLayer>) {
        if self.application.is_none() {
            self.application = Some(l);
        }
    }

    fn set_error_layer(&mut self, l: Rc<dyn ErrorLayer>) {
        if self.failure.is_none() {
            self.failure = Some(Rc::from(l));
        }
    }

    fn next_decoder(&mut self, next: Rc<DecodeFunc>) -> Result<(), DecodeError> {
        // Early return if next is None
        // Ensure there's a last layer
        let last_layer_opt = &self.last;

        let last_layer = match last_layer_opt {
            Some(layer) => layer,
            None => {
                return Err(NoLastLayerError::new(
                    "next_decoder called, but no last layers found",
                    None,
                ))
            }
        };

        // Ensure the last layer has a non-empty payload
        match last_layer.layer_payload() {
            None => return Ok(()),
            Some(payload) => {
                if payload.is_empty() {
                    return Ok(());
                }
                next(payload, Rc::new(RefCell::new(self.clone())))
            }
        }
    }

    fn dump_packet_data(&self) {
        eprintln!("{}", self.packet_dump());
    }

    fn decode_options(&self) -> DecodeOptions {
        self.decode_options
    }

    fn as_decode_feedback(&self) -> Rc<dyn DecodeFeedback> {
        Rc::new(self.clone())
    }

    fn layers_count(&self) -> usize {
        self.layers.len()
    }
}

impl EagerPacket {
    pub fn new(data: Rc<[u8]>, opts: DecodeOptions) -> Self {
        EagerPacket {
            data,
            initial_layers: vec![],
            layers: vec![],
            last: None,
            metadata: PacketMetadata {
                timestamp: SystemTime::now(), // Proper timestamp initialization,
                capture_length: 0,
                length: 0,
                interface_index: 0,
                ancillary_data: vec![],
                truncated: false,
            },
            decode_options: opts,
            link: None,
            network: None,
            transport: None,
            application: None,
            failure: None,
        }
    }

    pub fn initial_decode(&mut self, _dec: DecodeFunc) {}

    // This function would typically write to an error log in Rust, as writing directly to os.Stderr is less common

    pub fn metadata(&self) -> &PacketMetadata {
        &self.metadata
    }
}
impl Debug for EagerPacket {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Packetable for EagerPacket {
    fn string(&self) -> String {
        self.packet_string()
    }

    fn dump(&self) -> String {
        self.packet_dump()
    }

    // Returns a slice of references to the layers
    fn layers(&self) -> Vec<Rc<dyn Layer>> {
        self.layers.clone()
    }

    fn layer(&self, layer_type: LayerTypeID) -> Option<Rc<dyn Layer>> {
        self.layers
            .iter()
            .find(|layer| layer.layer_type().id == layer_type)
            .cloned() // Clone the found layer
    }

    fn layer_class(&self, layer_class: Rc<dyn LayerClass>) -> Option<Rc<dyn Layer>> {
        for layer in &self.layers {
            if layer_class.contains(layer.layer_type().id) {
                return Some(layer.clone());
            }
        }
        None
    }

    fn link_layer(&self) -> Option<Rc<dyn LinkLayer>> {
        self.link.clone()
    }

    // Accessor methods for each layer type
    fn network_layer(&self) -> Option<Rc<dyn NetworkLayer>> {
        self.network.clone()
    }

    fn transport_layer(&self) -> Option<Rc<dyn TransportLayer>> {
        self.transport.clone()
    }

    fn application_layer(&self) -> Option<Rc<dyn ApplicationLayer>> {
        self.application.clone()
    }

    fn error_layer(&self) -> Option<Rc<dyn ErrorLayer>> {
        self.failure.clone()
    }

    fn data(&self) -> Rc<[u8]> {
        self.data.clone()
    }

    fn metadata(&self) -> &PacketMetadata {
        &self.metadata
    }

    fn verify_checksums(&self) -> Result<Vec<ChecksumMismatch>, VerifyChecksumError> {
        let mut mismatches: Vec<ChecksumMismatch> = Vec::new();
        let layers = self.layers(); // Assuming this returns Vec<Rc<dyn Layer>>

        for (i, layer) in layers.iter().enumerate() {
            // Attempt to downcast layer to a specific layer trait if applicable
            match layer.verify_checksum() {
                Ok(cvr) => {
                    if !cvr.valid {
                        mismatches.push(ChecksumMismatch {
                            result: cvr,
                            layer: layer.clone(),
                            layer_index: i,
                        });
                    }
                }
                Err(err) => match err {
                    PacketError::MethodNotImplemented(_) => {
                        println!("layer does not verify checksum: {:?}", layer.layer_type())
                    }
                    PacketError::VerifyChecksum(err) => {
                        return Err(VerifyChecksumError::new(
                            &format!(
                                "could not verify checksum for layer {:?} ({:?}), {:?} ",
                                i + 1,
                                layer.layer_type(),
                                err
                            ),
                            Some(Box::new(err)),
                        ));
                    }
                    _ => {}
                },
            }
        }

        if mismatches.is_empty() {
            return Ok(mismatches);
        } else {
            return Err(VerifyChecksumError::new("Checksum mismatches found", None));
        }
    }

    fn packet_string(&self) -> String {
        let mut out = String::new();

        let _ = write!(out, "Packet: {} bytes", self.data.len());
        if self.metadata.truncated {
            let _ = write!(out, ", truncated");
        }
        if self.metadata.length > 0 {
            let _ = write!(
                out,
                ", wire length: {} cap length: {}",
                self.metadata.length, self.metadata.capture_length
            );
        }
        if let timestamp = self.metadata.timestamp {
            let _ = write!(out, " @ {:?}", timestamp);
        }
        let _ = write!(out, "\n");

        for (i, layer) in self.layers.iter().enumerate() {
            let _ = write!(
                out,
                "- Layer {} ({} bytes) = {}\n",
                i + 1,
                layer.layer_contents().unwrap().len(),
                "LayerString representation"
            );
        }

        out
    }
    fn packet_dump(&self) -> String {
        let mut out = String::new();

        // Header
        write!(
            out,
            "-- FULL PACKET DATA ({} bytes) ------------------------------------\n",
            self.data.len()
        )
        .unwrap();

        // Hex dump of the packet data
        let hex_dump = hex::encode(&self.data);
        write!(out, "{}\n", hex_dump).unwrap(); // Adjust formatting as needed

        // Iterate through layers
        for (i, layer) in self.layers.iter().enumerate() {
            let layer_dump = layer_dump(layer.clone());
            write!(out, "--- Layer {} ---\n{}\n", i + 1, layer_dump).unwrap();
        }

        out
    }

    // Special method to handle decode errors
    fn add_final_decode_error(&mut self, err: DecodeError) {
        let mut failure = DecodeFailure {
            layer_type: None,
            in_data: None,
            err: Rc::new(err),
            stack: vec![],
        };

        match self.last {
            Some(ref last) => failure.in_data = last.layer_payload(),
            None => failure.in_data = Some(self.data.clone()),
        };
        let rc_failure = Rc::new(failure);
        self.add_layer(rc_failure.clone());
        self.set_error_layer(rc_failure);
    }

    fn recover_decode_error(&mut self) {
        if !self.decode_options.skip_decode_recovery {
            let decode_error = DecodeError::new("recover decode error", None);
            self.add_final_decode_error(decode_error);
        }
    }
}

#[cfg(test)]
mod tests {

    // Assuming necessary traits and structs are defined elsewhere,
    // and mock implementations for Layer, LinkLayer, NetworkLayer, etc., are available.

    use std::rc::Rc;

    use crate::rtpacket::base::Layer;
    use crate::rtpacket::base::payload::Payload;
    use crate::rtpacket::decode::{DecodeFeedback, PacketBuilder};
    use crate::rtpacket::packet::decodeoptions::DecodeOptions;
    use crate::rtpacket::packet::eagerpacket::{convert_array_to_rc_slice, EagerPacket};
    use crate::rtpacket::packet::packetable::Packetable;

    #[test]
    fn test_verify_checksums() {
        let mut packet = EagerPacket::new(Rc::new([0u8; 0]), DecodeOptions::default());

        // Add mock layers to the packet
        let correct_layer = Rc::new(Payload::new_from(Rc::new([1u8, 2, 3, 4])));
        let incorrect_layer = Rc::new(Payload::new_from(Rc::new([5u8, 6, 7, 8])));
        packet.add_layer(correct_layer as Rc<dyn Layer>);
        packet.add_layer(incorrect_layer as Rc<dyn Layer>);

        // Invoke verify_checksums and assert the results
        match packet.verify_checksums() {
            Ok(mismatches) => {
                println!("Checksums Ok");
                assert_eq!(mismatches.len(), 0, "Expected 0 checksum mismatch.");
            }
            Err(err) => {
                panic!("Unexpected error: {} {:?}", err, err.source)
            }
        }
    }
    #[test]
    fn eager_packet_initialization() {
        let data: Rc<[u8]> = convert_array_to_rc_slice([0xdeu8, 0xad, 0xbe, 0xef]);
        let packet = EagerPacket::new(data.clone(), DecodeOptions::DEFAULT);

        assert_eq!(packet.data, data);
        assert_eq!(packet.decode_options, DecodeOptions::default());
        // Further assertions can be added as necessary to verify initial state.
    }

    #[test]
    fn set_truncated_flag() {
        let mut packet = EagerPacket::new(convert_array_to_rc_slice([]), DecodeOptions::default());
        assert!(
            !packet.metadata.truncated,
            "Packet should not be truncated initially."
        );

        packet.set_truncated();
        assert!(
            packet.metadata.truncated,
            "Packet should be marked as truncated after set_truncated call."
        );
    }

    // Example test for setting a layer
    #[test]
    fn set_link_layer_success() {
        let packet = EagerPacket::new(convert_array_to_rc_slice([]), DecodeOptions::default());
        assert!(
            packet.link.is_none(),
            "Link layer should be None initially."
        );

        // Assuming MockLinkLayer implements LinkLayer trait
        // let link_layer = Rc::new(MockLinkLayer::new());
        // packet.set_link_layer(link_layer.clone());

        // assert_eq!(packet.link, Some(link_layer), "Link layer should be set correctly.");
    }

    // Similar tests can be created for set_network_layer, set_transport_layer, etc.

    // Since many methods are not fully implemented (`todo!()`), specific tests for those will depend on their eventual implementation details.
}
