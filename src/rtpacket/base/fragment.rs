use std::error::Error;
use std::io;
use std::ops::Deref;
use std::rc::Rc;

use crate::rtpacket::base::{ApplicationLayer, Layer, Payloadable};
use crate::rtpacket::decode::{DecodeFeedback, decoder_builder, LayerType};
use crate::rtpacket::decode::decodefragment::fragment_decoder;
use crate::rtpacket::error::decodererror::DecodeError;
use crate::rtpacket::layerclass::LayerClass;
use crate::rtpacket::layertype::LayerTypeID;
use crate::rtpacket::layertype::LayerTypes::{LayerTypeFragment, LayerTypeZero};
use crate::rtpacket::writer::{
    SerializableLayer, SerializeableBuffer, SerializeBuffer, SerializeOptions,
};

// Structure representing a fragment of a larger frame.
#[derive(Clone)]
pub struct Fragment {
    layer_type: LayerType,
    in_data: Option<Rc<[u8]>>,
}

impl Fragment {
    /// Creates a new, empty `Fragment`.
    ///
    /// # Returns
    ///
    pub(crate) fn new() -> Fragment {
        Fragment {
            layer_type: LayerType {
                id: LayerTypeFragment as LayerTypeID,
                name: "DecodeFragment".to_owned(),
                decoder: fragment_decoder(),
            },
            in_data: None,
        }
    }

    /// Creates a new, `Fragment` from a vector.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to initialize the fragment with.
    ///
    /// # Returns
    ///
    /// A new `Fragment` containing the given data.
    pub(crate) fn new_from(data: Rc<[u8]>) -> Self {
        Fragment {
            layer_type: LayerType {
                id: LayerTypeFragment as LayerTypeID,
                name: "DecodeFragment".to_owned(),
                decoder: decoder_builder(LayerTypeFragment),
            },
            in_data: Some(data),
        }
    }
}

impl Layer for Fragment {
    fn layer_type(&self) -> LayerType {
        self.layer_type.clone()
    }

    fn layer_contents(&self) -> Option<Rc<[u8]>> {
        self.in_data.clone()
    }

    fn layer_payload(&self) -> Option<Rc<[u8]>> {
        None
    }

    fn string(&self) -> String {
        match &self.in_data {
            None => "0 byte(s)".to_string(),
            Some(data) => format!("{} byte(s)", data.deref().len()),
        }
    }
}

impl ApplicationLayer for Fragment {
    fn payload(&self) -> Option<Rc<[u8]>> {
        None
    }
}

impl SerializableLayer for Fragment {
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
                bytes.copy_from_slice(&data.deref());
                Ok(())
            }
        }
    }

    fn layer_type(&self) -> LayerType {
        self.layer_type.clone()
    }
}

impl Payloadable for Fragment {
    fn can_decode(&self) -> impl LayerClass {
        self.layer_type.clone()
    }

    fn next_layer_type(&self) -> LayerType {
        LayerType {
            id: LayerTypeZero as LayerTypeID,
            name: "LayerTypeZero".to_owned(),
            decoder: decoder_builder(LayerTypeZero),
        }
    }

    fn decode_from_bytes(
        &mut self,
        data: Rc<[u8]>,
        mut _builder: Box<dyn DecodeFeedback>,
    ) -> Result<(), DecodeError> {
        self.in_data = Option::from(data.clone());

        Ok(())
    }
}

// Implement Display trait for Fragment for a simple string representation.
impl std::fmt::Display for Fragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.in_data {
            None => write!(f, "0 byte(s)"),
            Some(data) => write!(f, "{} byte(s)", data.len()), // Uses automatic dereferencing
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rtpacket::base::payload::Payload;

    use super::*;

    const LAYER_TYPE_FRAGMENT: LayerTypeID = 3;

    #[test]
    fn test_payload_and_fragment_creation() {
        // Helper function to test a generic type that conforms to expected behavior
        fn test_creation<T: Payloadable>(new_instance: T, new_instance_from_data: T) {
            // Test the empty instance
            assert!(
                new_instance.layer_contents().is_none(),
                "Layer contents should be None for a new instance"
            );
            assert_eq!(
                new_instance.string(),
                "0 byte(s)",
                "String representation for an empty instance is incorrect"
            );

            // Test the instance created with specific data
            assert_eq!(
                new_instance_from_data.layer_contents().unwrap(),
                Rc::from(&[1, 2, 3][..]),
                "Layer contents should match the instance contents"
            );
            assert_eq!(
                new_instance_from_data.string(),
                "3 byte(s)",
                "String representation for a data-filled instance is incorrect"
            );

            let decoder = new_instance_from_data.can_decode();
            assert!(
                decoder.contains(LAYER_TYPE_FRAGMENT),
                "can_decode should return LAYER_TYPE_FRAGMENT"
            );

            let nt = new_instance_from_data.next_layer_type().id as isize;
            assert_eq!(
                nt, LayerTypeZero as isize,
                "next_layer_type should return LAYER_TYPE_ZERO"
            );
        }

        // Assuming both Fragment and Payload implement a trait CreationTestable that encapsulates
        // the required behaviors for testing
        let empty_fragment = Fragment::new();
        let filled_fragment = Fragment::new_from(Rc::from(&[1, 2, 3][..]));

        let empty_payload = Payload::new();
        let filled_payload = Payload::new_from(Rc::from(&[1, 2, 3][..]));

        // Run tests on both Fragment and Payload instances
        test_creation(empty_fragment, filled_fragment);
        test_creation(empty_payload, filled_payload);
    }

    #[test]
    fn test_fragment_creation() {
        // Case 1: Testing new fragment to be empty
        let empty_fragment = Fragment::new();
        assert!(
            empty_fragment.in_data.is_none(),
            "in_data should be None for a new fragment"
        );

        let layer_type = SerializableLayer::layer_type(&empty_fragment);
        assert_eq!(
            layer_type.id as isize, LayerTypeFragment as isize,
            "Layer type should be Fragment"
        );

        // The layer contents of a new fragment should also be None or empty
        assert!(
            empty_fragment.layer_contents().is_none(),
            "Layer contents should be None for a new fragment"
        );

        // Testing string representation for an empty fragment
        assert_eq!(
            empty_fragment.string(),
            "0 byte(s)",
            "String representation for an empty fragment is incorrect"
        );

        // Case 2: Testing fragment created with specific data
        let data_fragment = Fragment::new_from(Rc::from(&[1, 2, 3][..]));
        assert_eq!(
            data_fragment.layer_contents().unwrap(),
            Rc::from(&[1, 2, 3][..]),
            "Layer contents should match the fragment contents"
        );

        // Testing string representation for a data-filled fragment
        assert_eq!(
            data_fragment.string(),
            "3 byte(s)",
            "String representation for a data-filled fragment is incorrect"
        );

        // Testing decoding capabilities of the fragment
        let decoder = data_fragment.can_decode();
        assert!(
            decoder.contains(LAYER_TYPE_FRAGMENT),
            "can_decode should return LAYER_TYPE_FRAGMENT"
        );

        // Testing the next layer type for a fragment created with specific data
        let nt = data_fragment.next_layer_type();
        assert_eq!(
            nt.id as isize, LayerTypeZero as isize,
            "next_layer_type should return LAYER_TYPE_ZERO"
        );
    }

    #[test]
    fn serialize_to_success() {
        let fragment = Fragment::new_from(Rc::from(&[1, 2, 3][..]));
        let mut buffer = SerializeBuffer::new(); // Assuming a new method for simplicity
        let opts = SerializeOptions::default(); // Assuming default options for simplicity

        let result = fragment.serialize_to(&mut buffer, opts);
        assert!(
            result.is_ok(),
            "Serialization should succeed for non-empty data"
        );
        assert_eq!(
            buffer.bytes(),
            &[1, 2, 3],
            "Buffer contents did not match expected serialized data"
        );
    }

    #[test]
    fn serialize_to_failure_due_to_empty_data() {
        let fragment = Fragment::new(); // An empty fragment
        let mut buffer = SerializeBuffer::new();
        let opts = SerializeOptions::default();

        let result = fragment.serialize_to(&mut buffer, opts);
        assert!(
            result.is_err(),
            "Serialization should fail for empty fragment data"
        );

        if let Err(e) = result {
            assert_eq!(
                e.to_string(),
                "in_data is empty",
                "Error message did not match expected"
            );
        }
    }
}
