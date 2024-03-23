use std::error::Error;
use std::io;
use std::ops::Deref;
use std::rc::Rc;

use crate::rtpacket::base::{ApplicationLayer, Layer, Payloadable};
use crate::rtpacket::checksum::ChecksumVerificationResult;
use crate::rtpacket::decode::{DecodeFeedback, decoder_builder, LayerType};
use crate::rtpacket::error::{decodeerror, PacketError};
use crate::rtpacket::error::decodeerror::DecodeError;
use crate::rtpacket::error::ErrorDecodeable;
use crate::rtpacket::layerclass::LayerClass;
use crate::rtpacket::layertype::LayerTypeID;
use crate::rtpacket::layertype::LayerTypes::{
    LayerTypeDecodeFailure, LayerTypePayload, LayerTypeZero,
};
use crate::rtpacket::writer::{
    SerializableLayer, SerializeableBuffer, SerializeBuffer, SerializeOptions,
};

#[derive(Clone)]
pub struct Payload {
    layer_type: LayerType,
    in_data: Option<Rc<[u8]>>,
}

impl Payload {
    pub(crate) fn new() -> Self {
        Payload {
            layer_type: LayerType {
                id: LayerTypePayload as LayerTypeID,
                name: "Payload".to_owned(),
                decoder: decoder_builder(LayerTypePayload),
            },
            in_data: None, // or Vec::with_capacity(capacity) if you want to preallocate space.
        }
    }
    pub fn new_from(data: Rc<[u8]>) -> Self {
        Payload {
            layer_type: LayerType {
                id: LayerTypePayload as LayerTypeID,
                name: "Payload".to_owned(),
                decoder: decoder_builder(LayerTypePayload),
            },
            in_data: Option::from(data),
        }
    }
}

// Constructor function for Payload.

impl Layer for Payload {
    fn layer_type(&self) -> LayerType {
        self.layer_type.clone()
    }

    fn layer_contents(&self) -> Option<Rc<[u8]>> {
        self.in_data.clone()
    }

    fn layer_payload(&self) -> Option<Rc<[u8]>> {
        None
    }

    fn verify_checksum(&self) -> Result<ChecksumVerificationResult, PacketError> {
        Err(PacketError::try_from(decodeerror::DecodeError::new(
            "Payload layer does not have a checksum",
            None,
        ))
        .unwrap())
    }

    fn string(&self) -> String {
        match &self.in_data {
            None => "0 byte(s)".to_string(),
            Some(data) => format!("{} byte(s)", data.deref().len()),
        }
    }
}

impl ApplicationLayer for Payload {
    fn payload(&self) -> Option<Rc<[u8]>> {
        self.in_data.clone()
    }
}

impl SerializableLayer for Payload {
    /// Writes the serialized representation of this layer into the given buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer to write the serialized representation into.
    /// * `_opts` - Options for serializing the layer, unused in this implementation.
    ///
    /// # Returns
    ///
    /// This function returns a `Result<(), Box<dyn Error>>`, indicating the success
    /// or failure of writing the serialized representation into the buffer.
    fn serialize_to(
        &self,
        buffer: &mut SerializeBuffer,
        _opts: SerializeOptions,
    ) -> Result<(), Box<dyn Error>> {
        match &self.in_data {
            None => Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "in_data is empty",
            ))),
            Some(data) => {
                let size = data.deref().len();
                let bytes = buffer.prepend_bytes(size)?;
                bytes.copy_from_slice(data.deref());
                Ok(())
            }
        }
    }
    fn layer_type(&self) -> LayerType {
        self.layer_type.clone()
    }
}

impl Payloadable for Payload {
    fn can_decode(&self) -> impl LayerClass {
        self.layer_type.clone()
    }

    fn next_layer_type(&self) -> LayerType {
        LayerType {
            id: LayerTypeZero as LayerTypeID,
            name: "Unknown".to_owned(),
            decoder: decoder_builder(LayerTypeDecodeFailure), // Adjust based on how decoders are implemented.
        }
    }

    fn decode_from_bytes(
        &mut self,
        data: Rc<[u8]>,
        _decoder: Rc<dyn DecodeFeedback>,
    ) -> Result<(), DecodeError> {
        self.in_data = Option::from(data.clone());

        Ok(())
    }
}
