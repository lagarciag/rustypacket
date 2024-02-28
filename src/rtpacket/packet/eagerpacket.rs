use std::error::Error;
use std::fmt::Debug;
use std::rc::Rc;
use std::time::SystemTime;

use crate::rtpacket::base::{
    ApplicationLayer, ErrorLayer, Layer, LinkLayer, NetworkLayer, TransportLayer,
};
use crate::rtpacket::capture::PacketMetadata;
use crate::rtpacket::checksum::ChecksumMismatch;
use crate::rtpacket::decode::{DecodeFeedback, DecodeFunc, PacketBuilder};
use crate::rtpacket::layertype::LayerTypeID;
use crate::rtpacket::packet::DecodeOptions;
use crate::rtpacket::packet::Packetable;

//create_packet_struct!(EagerPacket);
pub struct EagerPacket {
    /// Contains the entire packet data.
    pub data: Vec<u8>,
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
    fn add_layer(&mut self, _l: Rc<dyn Layer>) {
        // let last = l.clone();
        // self.layers.push(l);
        // self.last = Some(last);
        todo!()
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

    fn set_error_layer(&mut self, l: Box<dyn ErrorLayer>) {
        if self.failure.is_none() {
            self.failure = Some(Rc::from(l));
        }
    }

    fn next_decoder(&mut self, _next: Rc<DecodeFunc>) -> Result<(), Box<dyn Error>> {
        /* // Early return if next is None

                // Ensure there's a last layer
                let last_layer_opt = self.last.as_ref();

                let last_layer = match last_layer_opt {
                    Some(layer) => layer,
                    None => {
                        return Err(Box::new(ErrorNoLastLayer::new(
                            "next_decoder called, but no last layers found",
                        )))
                    }
                };

                // Ensure the last layer has a non-empty payload
                match last_layer.layer_payload() {
                    None => return Ok(()),
                    Some(payload) => {
                        if payload.is_empty() {
                            return Ok(());
                        }
                        next(payload, self).map_err(|e| e)
                    }
                }
        */
        todo!()
    }

    fn dump_packet_data(&self) {
        todo!()
    }

    fn decode_options(&self) -> DecodeOptions {
        todo!()
    }

    fn as_decode_feedback(&self) -> Box<dyn DecodeFeedback> {
        //Box::new(self.clone())
        todo!()
    }

    fn layers_count(&self) -> usize {
        todo!()
    }
}

impl EagerPacket {
    pub(crate) fn new(data: Vec<u8>, opts: DecodeOptions) -> Self {
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

    pub(crate) fn initial_decode(&mut self, _dec: DecodeFunc) {
        todo!()
    }

    // This function would typically write to an error log in Rust, as writing directly to os.Stderr is less common

    pub fn metadata(&self) -> &PacketMetadata {
        &self.metadata
    }

    pub fn decode_options(&self) -> &DecodeOptions {
        &self.decode_options
    }

    // Special method to handle decode errors
    pub fn add_final_decode_error(&mut self, _err: Box<dyn std::error::Error>, stack: String) {
        // let failure_data = match self.last {
        //     Some(ref last) => last.layer_payload().to_vec(),
        //     None => self.data.clone(),
        // };
        //
        // let failure = Box::new(DecodeFailure {
        //     err,
        //     data: failure_data,
        //     stack,
        // });
        //
        // self.add_layer(failure.clone());
        // self.set_error_layer(failure);
        todo!()
    }

    pub fn recover_decode_error(&mut self) {
        todo!()
    }
}
impl Debug for EagerPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Packetable for EagerPacket {
    // Returns a slice of references to the layers
    fn layers(&self) -> Vec<Box<dyn Layer>> {
        //self.layers.iter().map(|l| l.as_ref() as &dyn Layer).collect()
        todo!()
    }

    fn layer(&self, layer_type: LayerTypeID) -> Option<Box<dyn Layer>> {
        // self.layers.iter()
        //     .find(|layer| layer.layer_type() == layer_type)
        //     .cloned() // Clone the found layer
        todo!()
    }

    fn layer_class(&self, lc: Box<dyn Layer>) -> Option<Box<dyn Layer>> {
        // for layer in &self.layers {
        //     if lc.contains(layer.layer_type()) {
        //         return Some(layer.Clone());
        //     }
        // }
        // None
        todo!()
    }

    fn link_layer(&self) -> Option<Box<dyn LinkLayer>> {
        //self.link.as_ref().map(|box_layer| box_layer.Clone())
        todo!()
    }

    // Accessor methods for each layer type
    fn network_layer(&self) -> Option<Box<dyn NetworkLayer>> {
        //self.network.Clone()
        todo!()
    }

    fn transport_layer(&self) -> Option<Box<dyn TransportLayer>> {
        //self.transport.Clone()
        todo!()
    }

    fn application_layer(&self) -> Option<Box<dyn ApplicationLayer>> {
        //self.application.Clone()
        todo!()
    }

    fn error_layer(&self) -> Option<Box<dyn ErrorLayer>> {
        //self.failure.Clone()
        todo!()
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn metadata(&self) -> &PacketMetadata {
        &self.metadata
    }

    fn verify_checksums(&self) -> Result<(), Vec<ChecksumMismatch>> {
        // let mut mismatches = Vec::new();
        // for (i, layer) in self.layers.iter().enumerate() {
        //     if let Layer::SomeChecksumLayer(ref lwc) = layer { // Assuming an enum variant for layers with checksums
        //         match lwc.verify_checksum() {
        //             Ok(res) if !res.valid => {
        //                 mismatches.push(ChecksumMismatch {
        //                     result: res,
        //                     layer: layer.clone(), // Assuming clone is implemented or use a reference
        //                     layer_index: i,
        //                 });
        //             }
        //             Err(e) => return Err(vec![e]), // Simplified error handling
        //             _ => {}
        //         }
        //     }
        // }
        //
        // if mismatches.is_empty() { Ok(()) } else { Err(mismatches) }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

// Assuming necessary traits and structs are defined elsewhere,
    // and mock implementations for Layer, LinkLayer, NetworkLayer, etc., are available.

    #[test]
    fn eager_packet_initialization() {
        let data = vec![0xde, 0xad, 0xbe, 0xef];
        let packet = EagerPacket::new(data.clone(), DecodeOptions::DEFAULT);

        assert_eq!(packet.data, data);
        assert_eq!(packet.decode_options, DecodeOptions::DEFAULT);
        // Further assertions can be added as necessary to verify initial state.
    }

    #[test]
    fn set_truncated_flag() {
        let mut packet = EagerPacket::new(vec![], DecodeOptions::default());
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
        let mut packet = EagerPacket::new(vec![], DecodeOptions::default());
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
