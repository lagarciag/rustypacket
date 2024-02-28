use crate::rtpacket::layertype::{LayerType, LayerTypeID, MAX_LAYER_TYPE};

/// Represents a set of `LayerType`s, used for selecting one of several
/// different types from a packet.
///
/// This trait allows for checking if a specific layer type is part of this
/// class and for retrieving all layer types within the class. Implementations
/// of this trait can represent groups of related network layers.
pub(crate) trait LayerClass {
    /// Determines if the given `LayerType` should be considered part of this layer class.
    ///
    /// # Arguments
    ///
    /// * `layer_type` - A `LayerType` to check against the class.
    ///
    /// # Returns
    ///
    /// * `true` if the `LayerType` is part of this class.
    /// * `false` otherwise.
    fn contains(&self, layer_type: LayerTypeID) -> bool;

    /// Returns a collection of all `LayerType`s in this class.
    ///
    /// This method provides access to the complete set of layer types that
    /// constitute this layer class. Implementors should note that this method
    /// may not be a fast operation in all cases.
    ///
    /// # Returns
    ///
    /// A vector of `LayerType` values representing all layer types in this class.
    fn layer_types(&self) -> Vec<LayerType>;
}

impl LayerClass for LayerType {
    fn contains(&self, a: LayerTypeID) -> bool {
        &self.id == &a
    }

    /// Returns itself as the only member of its class.
    ///
    /// Since `LayerType` represents a single layer type, this method
    /// returns a vector containing only the wrapped `LayerType`.
    ///
    /// # Returns
    ///
    /// A vector containing a single `LayerType`.
    fn layer_types(&self) -> Vec<LayerType> {
        vec![self.clone()]
    }
}

type LayerClassSlice = Vec<bool>;

impl LayerClass for LayerClassSlice {
    fn contains(&self, t: LayerTypeID) -> bool {
        /*
         self.get(t) attempts to get the element at index t.
         This method returns an Option<&bool>, safely handling cases where t might be out of bounds.
         copied() converts the Option<&bool> to Option<bool>, effectively cloning the underlying boolean value if it exists.
         unwrap_or(false) returns the contained value if Some, otherwise returns false.
         This ensures that if t is out of bounds (i.e., t >= len(s)) or if the value at index t is false,
         the method returns false.
         This Rust implementation closely mirrors the logic of the original Go code,
         checking if the LayerType t should be considered part of the LayerClassSlice by verifying if the value at index t in the vector is true.
        */
        self.get(t).copied().unwrap_or(false)
    }

    /// Returns all layer types in this LayerClassSlice.
    /// Because of LayerClassSlice's implementation, this could be quite slow.
    fn layer_types(&self) -> Vec<LayerType> {
        // self.iter()
        //     .enumerate()
        //     .filter_map(|(i, &is_in_class)| if is_in_class { Some(i) } else { None })
        //     .collect()
        todo!()
    }
}

/// Creates a new `LayerClassSlice` by creating a `Vec<bool>` of size `max(types) + 1`
/// and setting `vec[t]` to `true` for each type `t`. This allows for efficient membership
/// checks within the slice. Note, if you implement your own `LayerType` and give it a high value,
/// this WILL create a very large vector, potentially impacting performance.
///
/// # Arguments
///
/// * `types` - A slice of `LayerType` for which the `LayerClassSlice` is being created.
///
/// # Returns
///
/// A `Vec<bool>` representing the `LayerClassSlice`.
fn new_layer_class_slice(types: &[LayerTypeID]) -> Vec<bool> {
    let max = types.iter().max().cloned().unwrap_or_default();
    let mut t = vec![false; max + 1];
    for &typ in types {
        t[typ] = true;
    }
    t
}

/// `LayerClassMap` implements a `LayerClass` with a map.
/// It maps `LayerType` to a boolean value, allowing for a flexible and dynamic
/// representation of layer classes. This structure is useful for scenarios where
/// layer membership needs to be quickly checked or updated, providing an efficient
/// means to handle layer types dynamically at runtime.
type LayerClassMap = std::collections::HashMap<LayerTypeID, bool>;

impl LayerClass for LayerClassMap {
    /// Returns `true` if the given layer type is considered part of this layer class.
    ///
    /// # Arguments
    ///
    /// * `t` - The `LayerType` to check for membership.
    ///
    /// # Returns
    ///
    /// `true` if `t` is part of this class, `false` otherwise.
    fn contains(&self, t: LayerTypeID) -> bool {
        *self.get(&t).unwrap_or(&false)
    }

    /// Returns all `LayerType`s contained within this `LayerClassMap`.
    ///
    /// # Returns
    ///
    /// A vector of all `LayerType`s in this class.
    fn layer_types(&self) -> Vec<LayerType> {
        // self.iter()
        //     .filter_map(|(t, &v)| if v { Some(*t) } else { None })
        //     .collect()
        todo!()
    }
}

/// Creates a new `LayerClassMap` with each given layer type set to `true`.
///
/// This function initializes a `LayerClassMap` and marks each `LayerType` from the
/// provided slice as present within the map. This is useful for categorizing
/// layer types into groups or classes for packet processing or identification.
///
/// # Arguments
///
/// * `types` - A slice of `LayerType` indicating which types should be included
///             in the new map.
///
/// # Returns
///
/// Returns a `LayerClassMap` where each type from `types` is set to `true`.
pub fn new_layer_class_map(types: &[LayerTypeID]) -> LayerClassMap {
    let mut m = LayerClassMap::new();
    for &typ in types {
        m.insert(typ, true);
    }
    m
}

/// Creates a `LayerClass`, choosing the most efficient storage based on the
/// provided layer types.
///
/// If any of the provided layer types exceed `MAX_LAYER_TYPE`, indicating potential
/// for high memory usage with a slice-based approach, a `LayerClassMap` is used for
/// efficient storage. Otherwise, a `LayerClassSlice` is used.
///
/// # Arguments
///
/// * `types` - A slice of `LayerType` indicating which types should be included
///             in the new layer class.
///
/// # Returns
///
/// Returns a `LayerClass`, which could be either a `LayerClassMap` or `LayerClassSlice`,
/// based on the provided types for efficient storage and access.
pub fn new_layer_class(types: &[LayerTypeID]) -> Box<dyn LayerClass> {
    if types.iter().any(|&typ| typ > MAX_LAYER_TYPE) {
        // Use a map-based approach if any type exceeds the max allowed value,
        // to avoid creating a very large slice.
        Box::new(new_layer_class_map(types))
    } else {
        // Otherwise, a slice-based approach is sufficient and potentially more
        // efficient for smaller ranges of types.
        Box::new(new_layer_class_slice(types))
    }
}

#[cfg(test)]
mod tests {
    use crate::rtpacket::layerclass::new_layer_class;
    use crate::rtpacket::layertype::MAX_LAYER_TYPE;

    #[test]
    fn test_new_layer_class_with_map() {
        // This test verifies that a LayerClassMap is created when layer types exceed MAX_LAYER_TYPE.

        // Define layer types that exceed MAX_LAYER_TYPE.
        let types = vec![MAX_LAYER_TYPE + 1, MAX_LAYER_TYPE + 2];

        // Call new_layer_class with these types.
        let layer_class = new_layer_class(&types);

        // Verify that the returned LayerClass is a LayerClassMap.
        // Note: This kind of type checking is not straightforward in Rust because of dynamic dispatch.
        // One approach is to try downcasting and checking if it succeeds, but that requires the LayerClass trait to be object-safe and implement Any.
        // For this example, we'll simply check if it contains the expected types, assuming the implementations for LayerClassMap and LayerClassSlice are known.

        // Assuming both implementations have a method to check if a type is contained.
        assert!(layer_class.contains(types[0]));
        assert!(layer_class.contains(types[1]));
    }

    #[test]
    fn test_new_layer_class_with_slice() {
        // This test verifies that a LayerClassSlice is created when all layer types are below MAX_LAYER_TYPE.

        // Define layer types that do not exceed MAX_LAYER_TYPE.
        let types = vec![1, 2, 3];

        // Call new_layer_class with these types.
        let layer_class = new_layer_class(&types);

        // Verify that the returned LayerClass is a LayerClassSlice by checking if it contains the expected types.
        assert!(layer_class.contains(types[0]));
        assert!(layer_class.contains(types[1]));
        assert!(layer_class.contains(types[2]));
    }
}
