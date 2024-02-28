use std::fmt::Debug;

use crate::rtpacket::base::{
    ApplicationLayer, ErrorLayer, Layer, LinkLayer, NetworkLayer, TransportLayer,
};
use crate::rtpacket::capture::PacketMetadata;
use crate::rtpacket::checksum::ChecksumMismatch;
use crate::rtpacket::layertype::LayerTypeID;
use crate::rtpacket::packet::decodeoptions::DecodeOptions;

/// Represents the primary object used by a packet processing library. Packets are created
/// by a `Decoder`'s decode call. A packet consists of a set of data, which
/// is broken into a number of layers as it is decoded.
pub trait Packetable: Debug {
    /// Returns a human-readable string representation of the packet.
    /// It uses `layer_string` on each layer to output the layer.
    fn string(&self) -> String {
        self.packet_string()
    }

    /// Returns a verbose human-readable string representation of the packet,
    /// including a hex dump of all layers. It uses `layer_dump` on each layer to
    /// output the layer.
    fn dump(&self) -> String {
        self.packet_dump()
    }

    /// Returns all layers in this packet, computing them as necessary.
    fn layers(&self) -> Vec<Box<dyn Layer>>;

    /// Returns the first layer in this packet of the given type, or `None`.
    fn layer(&self, layer_type: LayerTypeID) -> Option<Box<dyn Layer>>;

    /// Returns the first layer in this packet of the given class, or `None`.
    fn layer_class(&self, layer_class: Box<dyn Layer>) -> Option<Box<dyn Layer>>;

    /// Returns the first link layer in the packet.
    fn link_layer(&self) -> Option<Box<dyn LinkLayer>>;

    /// Returns the first network layer in the packet.
    fn network_layer(&self) -> Option<Box<dyn NetworkLayer>>;

    /// Returns the first transport layer in the packet.
    fn transport_layer(&self) -> Option<Box<dyn TransportLayer>>;

    /// Returns the first application layer in the packet.
    fn application_layer(&self) -> Option<Box<dyn ApplicationLayer>>;

    /// Returns `None` if the packet was fully decoded successfully, and
    /// `Some` if an error was encountered in decoding and the packet was only
    /// partially decoded. Thus, its output can be used to determine if the
    /// entire packet was able to be decoded.
    fn error_layer(&self) -> Option<Box<dyn ErrorLayer>>;

    /// Returns the set of bytes that make up this entire packet.
    fn data(&self) -> &[u8];

    /// Returns packet metadata associated with this packet.
    fn metadata(&self) -> &PacketMetadata;

    /// Verifies the checksums of all layers in this packet that have one, and
    /// returns all found checksum mismatches.
    fn verify_checksums(&self) -> Result<(), Vec<ChecksumMismatch>>;
    // Constructs a new EagerPacket and eagerly decodes all layers.

    fn packet_string(&self) -> String {
        // let mut result = format!("PACKET: {} bytes", self.data().len());
        // if self.metadata().truncated {
        //     result.push_str(", truncated");
        // }
        // if self.metadata().length > 0 {
        //     result.push_str(&format!(", wire length {} cap length {}",
        //                              self.metadata().length,
        //                              self.metadata().capture_length));
        // }
        // if let Some(timestamp) = self.metadata().timestamp {
        //     // Formatting the timestamp as RFC3339, similar to Go's default
        //     result.push_str(&format!(" @ {}", timestamp.format("%+").to_string()));
        // }
        // result.push('\n');
        //
        // for (i, layer) in self.layers().iter().enumerate() {
        //     result.push_str(&format!("- Layer {} ({:02} bytes) = {}\n", i + 1, layer.layer_contents().len(), layer.layer_type()));
        //     // Assuming LayerString functionality is implemented via Display trait for layers
        // }
        // result
        todo!()
    }

    // Generates a detailed dump of the packet, including a hex dump of the packet data
    /// and a detailed dump of each layer.
    ///
    /// Returns a string representation of the packet dump.
    fn packet_dump(&self) -> String {
        // let mut result = String::new();
        //
        // // Adding packet data length and hex dump
        // writeln!(
        //     result,
        //     "-- FULL PACKET DATA ({} bytes) ------------------------------------",
        //     self.data().len()
        // )
        //     .unwrap();
        // result.push_str(&hex_encode(&self.data())); // Assume hex_dump is a function that returns a hex dump string
        //
        // // Iterating over layers to add their dumps
        // for (i, layer) in self.layers().iter().enumerate() {
        //     writeln!(result, "--- Layer {} ---", i + 1).unwrap();
        //     result.push_str(&layer.layer_dump()); // Assume layer_dump is a method provided by the Layer trait
        // }
        //
        // result
        todo!()
    }

    fn dump_packet_data(&self) {
        eprintln!("{:?}", self.packet_dump()); // Assuming packet_dump is implemented
    }
}

pub trait PooledPacket: Packetable {
    /// Disposes of the packet, returning it to a pool or performing any necessary cleanup.
    fn dispose(&mut self);
}

// Contains all the information required to fulfill the `Packet` trait.
// This struct, along with its associated types (`EagerPacket` and `LazyPacket`),
// provide eager and lazy decoding logic around the various functions needed
// to access packet information.
pub struct Packet {
    /// Contains the entire packet data.
    pub data: Vec<u8>,
    /// Space for an initial set of layers already created inside the packet.
    pub initial_layers: Vec<Box<dyn Layer>>,
    /// Contains each layer that has been decoded.
    pub layers: Vec<Box<dyn Layer>>,
    /// The last layer added to the packet.
    pub last: Option<Box<dyn Layer>>,
    /// Metadata associated with this packet.
    pub metadata: PacketMetadata,
    /// Decoding options for this packet.
    pub decode_options: DecodeOptions,

    /// Pointer to the various important layers.
    pub link: Option<Box<dyn LinkLayer>>,
    pub network: Option<Box<dyn NetworkLayer>>,
    pub transport: Option<Box<dyn TransportLayer>>,
    pub application: Option<Box<dyn ApplicationLayer>>,
    pub failure: Option<Box<dyn ErrorLayer>>,
}
impl Packet {
    pub fn set_truncated(&mut self) {
        self.metadata.truncated = true;
    }

    pub fn set_link_layer(&mut self, l: Box<dyn LinkLayer>) {
        if self.link.is_none() {
            self.link = Some(l);
        }
    }

    pub fn set_network_layer(&mut self, l: Box<dyn NetworkLayer>) {
        if self.network.is_none() {
            self.network = Some(l);
        }
    }

    pub fn set_transport_layer(&mut self, l: Box<dyn TransportLayer>) {
        if self.transport.is_none() {
            self.transport = Some(l);
        }
    }

    pub fn set_application_layer(&mut self, l: Box<dyn ApplicationLayer>) {
        if self.application.is_none() {
            self.application = Some(l);
        }
    }

    pub fn set_error_layer(&mut self, l: Box<dyn ErrorLayer>) {
        if self.failure.is_none() {
            self.failure = Some(l);
        }
    }

    pub fn add_layer(&mut self, _l: Box<dyn Layer>) {
        // let last = l.clone();
        // self.layers.push(l);
        // self.last = Some(last);
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
    pub fn add_final_decode_error(&mut self, err: Box<dyn std::error::Error>, stack: String) {
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
impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Packetable for Packet {
    fn layers(&self) -> Vec<Box<dyn Layer>> {
        //self.layers.clone()
        todo!()
    }

    //fn layer(&self, layer_type: LayerType) -> Option<Box<dyn Layer>>;
    fn layer(&self, layer_type: LayerTypeID) -> Option<Box<dyn Layer>> {
        todo!()
    }
    fn layer_class(&self, lc: Box<dyn Layer>) -> Option<Box<dyn Layer>> {
        // for layer in &self.layers {
        //     if lc.contains(layer.layer_type()) {
        //         return Some(layer.as_ref());
        //     }
        // }
        // None
        todo!()
    }
    fn link_layer(&self) -> Option<Box<dyn LinkLayer>> {
        //self.link.as_ref().map(|box_layer| box_layer.Clone())
        todo!()
    }

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

    // Assuming LayerClass is a trait with a method contains that checks if a LayerType is part of the class

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
