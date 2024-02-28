use std::error::Error;
use std::fmt;

use crate::rtpacket::layertype::LayerType;

/// A trait for types that can be serialized into a byte representation.
///
/// This trait allows its implementations to be written out as a set of bytes,
/// so those bytes may be sent on the wire or otherwise used by the caller.
/// `SerializableLayer` is implemented by certain layer types, and can be encoded to
/// bytes using a `LayerWriter` object.
///
/// Implementations of `SerializableLayer` should provide the logic to serialize the
/// layer's data onto a `SerializeBuffer`.
pub trait SerializableLayer {
    /// Serializes this layer onto the given `SerializeBuffer`, potentially growing
    /// the buffer if necessary to fit the layer's data.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A mutable reference to a `SerializeBuffer` onto which this layer
    ///   will be written. Initially, `buffer.bytes()` represents the payload that
    ///   this layer should wrap, if any. Implementations can prepend, append, or
    ///   both modify the payload. It's also possible to overwrite any part of
    ///   the current payload, though this is rare.
    ///
    /// * `options` - Serialization options to use while writing out data.
    ///
    /// # Returns
    ///
    /// This method returns a `Result<(), Box<dyn Error>>` to indicate success or
    /// failure. If an error is returned, the contents of `buffer` should be
    /// considered invalidated and not used.
    ///
    /// # Note
    ///
    /// Implementations should entirely ignore `LayerContents` and `LayerPayload`,
    /// focusing solely on serializing the layer based on its struct fields without
    /// modifying or using the contents/payload directly.
    fn serialize_to(
        &self,
        buffer: &mut SerializeBuffer,
        options: SerializeOptions,
    ) -> Result<(), Box<dyn Error>>;

    /// Returns the type of the layer that is being serialized.
    ///
    /// This method should provide the specific type of the layer, which can be used
    /// to identify the kind of layer being processed or serialized.
    ///
    /// # Returns
    ///
    /// A `LayerType` indicating the type of the layer.
    fn layer_type(&self) -> LayerType;
}

/// Options for controlling serialization behavior of `SerializableLayer` implementations.
///
/// This struct provides options that can influence how layers are serialized,
/// allowing for customization of certain aspects of the serialization process.
#[derive(Debug, Clone, Copy, Default)]
pub struct SerializeOptions {
    /// Determines whether layer lengths should be automatically adjusted during
    /// serialization to match the payload size.
    ///
    /// When set to `true`, any field representing the length of a layer or the
    /// length of its payload will be recalculated to ensure accuracy. This is
    /// useful for protocols where the length of a packet or header needs to be
    /// specified and might change due to dynamic content.
    pub fix_lengths: bool,

    /// Determines whether checksums should be recalculated based on the layer's
    /// payload during serialization.
    ///
    /// Setting this to `true` triggers a recomputation of checksums for layers
    /// that include such validation mechanisms. This ensures that serialized
    /// data maintains integrity and conforms to protocol specifications that
    /// require checksum validation.
    pub compute_checksums: bool,
}

/// A trait for a buffer that supports serialization of packet layers.
///
/// `SerializeBuffer` is designed to facilitate packet writing, particularly
/// allowing for easy prepending of bytes, which is common in network packet
/// construction. This trait manages a buffer that can grow both forwards and
/// backwards, accommodating the need to add data both before and after existing data.
///
/// Note that clearing the buffer invalidates any references to the data it contained.
pub trait SerializeableBuffer {
    /// Returns a slice to the current bytes in the buffer.
    ///
    /// The returned slice will be modified by future calls to `clear`, so it's
    /// advisable to copy the data elsewhere if the buffer will be cleared.
    fn bytes(&self) -> &[u8];

    /// Prepends the specified number of bytes to the start of the buffer.
    ///
    /// The new bytes are uninitialized and must be immediately overwritten by the caller.
    /// The method returns a mutable slice to the prepended bytes.
    ///
    /// # Arguments
    ///
    /// * `num` - The number of bytes to prepend.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation cannot be completed, e.g., due
    /// to allocation failure.
    fn prepend_bytes(&mut self, num: usize) -> Result<&mut [u8], Box<dyn Error>>;

    /// Appends the specified number of bytes to the end of the buffer.
    ///
    /// The new bytes are uninitialized and must be immediately overwritten by the caller.
    /// The method returns a mutable slice to the appended bytes.
    ///
    /// # Arguments
    ///
    /// * `num` - The number of bytes to append.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation cannot be completed, e.g., due to allocation failure.
    fn append_bytes(&mut self, num: usize) -> Result<&mut [u8], Box<dyn std::error::Error>>;

    /// Clears the buffer, resetting it to an empty state.
    ///
    /// After calling `clear`, any data previously in the buffer is lost, and
    /// slices returned by `bytes` are invalidated.
    fn clear(&mut self);

    /// Returns a list of all layer types that have been successfully serialized into this buffer.
    fn layers(&self) -> Vec<LayerType>;

    /// Records the serialization of a layer of the specified type into this buffer.
    ///
    /// # Arguments
    ///
    /// * `layer_type` - The type of the layer that was serialized.
    fn push_layer(&mut self, layer_type: LayerType);
}

#[derive(Clone)]
pub struct SerializeBuffer {
    data: Vec<u8>,
    // Replaces []byte
    start: usize,
    // Equivalent to 'int' but more precise in Rust; usize is commonly used for indexing
    prepended: usize,
    // Replaces 'prepended int'
    appended: usize,
    // Replaces 'appended int'
    layers: Vec<LayerType>, // Assuming LayerType is already defined somewhere
}

impl SerializeBuffer {
    /// Creates a new `SerializeBuffer` with default settings.
    /// This function initializes the buffer with no preallocated space for
    /// prepend or append operations, suitable for use cases where the size
    /// of operations is not known in advance.
    pub(crate) fn new() -> Self {
        SerializeBuffer {
            data: Vec::new(), // No preallocated space, will grow as needed.
            start: 0,         // Since there's no preallocation, start is 0.
            prepended: 0,
            appended: 0,
            layers: vec![],
        }
    }

    /// Creates a new `SerializeBuffer` with specified expected sizes for prepending and appending.
    /// This helps in optimizing memory allocations.
    ///
    /// # Arguments
    ///
    /// * `expected_prepend_length` - The expected number of bytes to prepend.
    /// * `expected_append_length` - The expected number of bytes to append.
    fn new_default(expected_prepend_length: usize, expected_append_length: usize) -> Self {
        // Preallocate buffer size based on expected prepend and append lengths.
        let capacity = expected_prepend_length + expected_append_length;
        let mut buffer = Vec::with_capacity(capacity);

        // Initialize the buffer with zeros for the expected prepend length to simulate
        // the space where data will be prepended. This ensures that the prepend operation
        // can be done efficiently.
        buffer.resize(expected_prepend_length, 0u8);

        SerializeBuffer {
            data: buffer,
            start: expected_prepend_length,
            prepended: 0,
            appended: 0,
            layers: vec![],
        }
    }
}

impl<'a> SerializeableBuffer for SerializeBuffer {
    /// Returns a slice to the bytes in the buffer that contains any data written.
    /// This slice starts from the `start` position, effectively skipping any preallocated
    /// space meant for prepending data.
    fn bytes(&self) -> &[u8] {
        &self.data[self.start..]
    }

    /// Prepends the specified number of bytes to the start of the buffer.
    ///
    /// The new bytes are uninitialized and must be immediately overwritten by the caller.
    /// The method returns a mutable slice to the prepended bytes.
    ///
    /// # Arguments
    ///
    /// * `num` - The number of bytes to prepend.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation cannot be completed, e.g., due to allocation failure.
    fn prepend_bytes(&mut self, num: usize) -> Result<&mut [u8], Box<dyn Error>> {
        if self.start < num {
            // The number of bytes to prepend.
            let mut to_prepend = self.prepended;
            // If the number of bytes to prepend is less than the specified number, set it to the specified number.
            if to_prepend < num {
                to_prepend = num;
            }
            // Increment the number of prepended bytes.
            self.prepended += to_prepend;
            // Calculate the new length of the data vector, including the number of prepended bytes.
            let length = self.data.capacity() + to_prepend;
            // Create a new vector with the calculated length and fill it with zeros.
            let mut new_data = vec![0u8; length];
            // Calculate the new start position, including the number of prepended bytes.
            let new_start = self.start + to_prepend;
            // Copy the existing data into the new vector, starting from the current start position.
            new_data[new_start..].copy_from_slice(&self.data[self.start..]);
            // Update the start position to include the number of prepended bytes.
            self.start += new_start;
            // Update the data vector to include the prepended bytes.
            self.data = new_data[..to_prepend + self.data.len()].to_owned();
        }
        // Set the start position to the current position minus the number of prepended bytes.
        self.start -= num;
        // Return a mutable slice to the prepended bytes.
        Ok(&mut self.data[self.start..self.start + num])
    }

    fn append_bytes(&mut self, num: usize) -> Result<&mut [u8], Box<dyn Error>> {
        let initial_length = self.data.len();
        if self.data.capacity() - initial_length < num {
            let to_append = std::cmp::max(self.appended, num);
            self.appended += to_append;
            self.data.reserve(to_append);
        }
        self.data.resize(initial_length + num, 0u8);
        Ok(&mut self.data[initial_length..])
    }

    /// Clears the given write buffer, then serializes and writes all provided layers into it
    /// such that they correctly wrap each other. It's important to note that by clearing
    /// the buffer, it invalidates all slices previously returned by the buffer's `bytes` method.
    ///
    ///
    /// # Arguments
    ///
    /// * `buffer` - A mutable reference to the buffer implementing the `SerializeBuffer` trait where layers will be serialized into.
    /// * `options` - Serialization options provided as an instance of `SerializeOptions`.
    /// * `layers` - A slice of references to objects implementing the `SerializableLayer` trait, representing the layers to serialize.
    ///
    /// # Returns
    ///
    /// This function returns a `Result<(), Box<dyn Error>>`. On success, it returns `Ok(())`.
    /// On failure, it returns an `Err` with the error that occurred during serialization.
    fn clear(&mut self) {
        self.start = self.prepended;
        self.data = self.data[..self.start].to_owned();
        self.layers = vec![];
    }

    fn layers(&self) -> Vec<LayerType> {
        self.layers.clone()
    }

    fn push_layer(&mut self, layer_type: LayerType) {
        self.layers.push(layer_type);
    }
}

/// Clears the given write buffer, then serializes and writes all provided layers into it
/// such that they correctly wrap each other. It's important to note that by clearing
/// the buffer, it invalidates all slices previously returned by the buffer's `bytes` method.
///
/// # Example
///
/// # Arguments
///
/// * `buffer` - A mutable reference to the buffer implementing the `SerializeBuffer` trait where layers will be serialized into.
/// * `options` - Serialization options provided as an instance of `SerializeOptions`.
/// * `layers` - A slice of references to objects implementing the `SerializableLayer` trait, representing the layers to serialize.
///
/// # Returns
///
/// This function returns a `Result<(), Box<dyn Error>>`. On success, it returns `Ok(())`.
/// On failure, it returns an `Err` with the error that occurred during serialization.
pub fn serialize_layers(
    buffer: &mut SerializeBuffer,
    options: SerializeOptions,
    layers: &[Box<dyn SerializableLayer>],
) -> Result<(), Box<dyn Error>> {
    buffer.clear();
    for layer in layers.iter().rev() {
        layer.serialize_to(buffer, options.clone())?;
        buffer.push_layer(layer.layer_type());
    }
    Ok(())
}

// pub fn serialize_packet(buffer: &mut dyn SerializeableBuffer, options: SerializeOptions, packet: &Packet) -> Result<(), Box<dyn Error>> {
//     let mut serializable_layers: Vec<Box<dyn SerializableLayer>> = Vec::new();
//
//     for layer in packet.layers() {
//         if let Some(sl) = layer.as_any().downcast_ref::<Box<dyn SerializableLayer>>() {
//             serializable_layers.push(sl.clone());
//         } else {
//             return Err(Box::new(SerializeError::new(format!("Layer {:?} is not serializable", layer.layer_type()))));
//         }
//     }
//
//     serialize_layers(buffer, options, &serializable_layers)
// }

// Error type for serialization errors
#[derive(Debug)]
struct SerializeError {
    message: String,
}

impl SerializeError {
    fn new(message: String) -> SerializeError {
        SerializeError { message }
    }
}

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SerializeError: {}", self.message)
    }
}

impl Error for SerializeError {}

#[cfg(test)]
mod tests {
    use super::*;

// Adjust this to import your SerializeBuffer struct

    #[test]
    fn test_exponential_size_increase_prepend() {
        let mut b = SerializeBuffer::new(); // Assuming a constructor method .new()
        let tests = [
            (2, 2),
            (2, 4),
            (2, 8),
            (2, 8),
            (2, 16),
            (2, 16),
            (2, 16),
            (2, 16),
            (2, 32),
        ];

        for (i, &(prepend, size)) in tests.iter().enumerate() {
            let err = b.prepend_bytes(prepend);
            match err {
                Ok(_) => {
                    assert_eq!(
                        b.data.capacity(),
                        size,
                        "At iteration {}: expected size {}, got {}",
                        i,
                        size,
                        b.data.capacity()
                    );
                }
                Err(_) => {
                    panic!(
                        "At iteration {}: expected size {}, got {}",
                        i,
                        size,
                        b.data.capacity()
                    );
                }
            }
        }

        b.clear(); // Assuming clear is similar to Clear
        assert_eq!(
            b.start, 32,
            "Expected start to be 32 after clear, got {}",
            b.start
        );
    }

    #[test]
    fn test_exponential_size_increase_append() {
        let mut b = SerializeBuffer::new(); // Assuming this is how you create a new instance
        let tests = vec![
            (2, 8),
            (2, 8),
            (2, 8),
            (2, 8),
            (2, 16),
            (2, 16),
            (2, 16),
            (2, 16),
            (2, 32),
            (2, 32),
        ];

        for (i, (appnd, size)) in tests.iter().enumerate() {
            b.append_bytes(*appnd).expect("TODO: panic message");
            assert_eq!(
                b.data.capacity(),
                *size,
                "Test {}: Expected size {}, got {}",
                i,
                size,
                b.data.capacity()
            );
        }

        b.clear();
        assert_eq!(b.start, 0, "Expected start position 0, got {}", b.start);
    }
}
