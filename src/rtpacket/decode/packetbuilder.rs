use std::rc::Rc;

use crate::rtpacket::base::{
    ApplicationLayer, ErrorLayer, Layer, LinkLayer, NetworkLayer, TransportLayer,
};
use crate::rtpacket::decode::{DecodeFeedback, DecodeFunc};
use crate::rtpacket::error::decodeerror::DecodeError;
use crate::rtpacket::packet::decodeoptions::DecodeOptions;

/// Used by layer decoders to store the layers they've decoded,
/// and to defer future decoding via `next_decoder`.
///
/// Implementors of this trait should ensure that `next_decoder` is called
/// only after all other relevant method calls are made, as it signifies
/// the end of processing for the current layer.
pub trait PacketBuilder: DecodeFeedback {
    /// Called immediately upon successful decoding of a layer.
    fn add_layer(&mut self, layer: Rc<dyn Layer>);

    /// Sets the specific layer types in the final packet.
    /// Note: Only the first call to each `set_*` method takes effect; subsequent calls are ignored.
    fn set_link_layer(&mut self, layer: Rc<dyn LinkLayer>);
    fn set_network_layer(&mut self, layer: Rc<dyn NetworkLayer>);
    fn set_transport_layer(&mut self, layer: Rc<dyn TransportLayer>);
    fn set_application_layer(&mut self, layer: Rc<dyn ApplicationLayer>);
    fn set_error_layer(&mut self, layer: Rc<dyn ErrorLayer>);

    /// Should be called by a decoder when it's done decoding a packet layer
    /// but further decoding is required. The provided `next` decoder is then
    /// used to decode the last added layer's payload.
    fn next_decoder(&mut self, next: Rc<DecodeFunc>) -> Result<(), DecodeError>;

    /// Utility method for debugging. Should dump packet data to stderr or
    /// another diagnostic output. Not intended for use in production decoders.
    fn dump_packet_data(&self);

    /// Returns the decode options associated with this packet builder.
    fn decode_options(&self) -> DecodeOptions;

    fn as_decode_feedback(&self) -> Rc<dyn DecodeFeedback>;

    fn layers_count(&self) -> usize;
}

// A mock implementation of the PacketBuilder trait for testing purposes
#[derive(Clone)]
pub(crate) struct MockPacketBuilder {
    pub(crate) layers_added: Vec<Rc<dyn Layer>>,
    pub(crate) link_layer: Option<Rc<dyn LinkLayer>>,
    pub(crate) application_layer: Option<Rc<dyn ApplicationLayer>>,
}

impl MockPacketBuilder {}

impl DecodeFeedback for MockPacketBuilder {
    fn set_truncated(&mut self) {
        todo!()
    }
}

impl PacketBuilder for MockPacketBuilder {
    fn add_layer(&mut self, layer: Rc<dyn Layer>) {
        self.layers_added.push(layer);
    }

    fn set_link_layer(&mut self, layer: Rc<dyn LinkLayer>) {
        self.link_layer = Some(layer);
    }

    fn set_network_layer(&mut self, _layer: Rc<dyn NetworkLayer>) {
        todo!()
    }

    fn set_transport_layer(&mut self, _layer: Rc<dyn TransportLayer>) {
        todo!()
    }

    fn set_application_layer(&mut self, layer: Rc<dyn ApplicationLayer>) {
        self.application_layer = Some(layer);
    }

    fn set_error_layer(&mut self, _layer: Rc<dyn ErrorLayer>) {
        todo!()
    }

    fn next_decoder(&mut self, _next: Rc<DecodeFunc>) -> Result<(), DecodeError> {
        todo!()
    }

    fn dump_packet_data(&self) {
        todo!()
    }

    fn decode_options(&self) -> DecodeOptions {
        todo!()
    }

    fn as_decode_feedback(&self) -> Rc<dyn DecodeFeedback> {
        Rc::new(self.clone())
    }

    fn layers_count(&self) -> usize {
        self.layers_added.len()
    }
}
