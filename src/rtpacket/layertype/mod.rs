use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use std::sync::Mutex;

use crate::rtpacket::decode::{DecodeFunc, PacketBuilder};
use crate::rtpacket::error::decodeerror::DecodeError;

const ARRAY_REPEAT_VALUE: Option<LayerType> = None;

#[derive(Clone, Debug)]
pub struct LayerType {
    pub id: LayerTypeID,
    /// The name of the layer, returned by each layer type's String method.
    pub name: String,
    /// The decoder to use for the layer type.
    pub decoder: DecodeFunc,
}

impl LayerType {
    fn decode(
        &self,
        data: Rc<[u8]>,
        builder: Rc<RefCell<dyn PacketBuilder>>,
    ) -> Result<(), DecodeError> {
        (self.decoder)(data, builder)
    }
}

// Necessary for initializing global state
//
/// Represents a unique identifier for each type of layer within this library.
/// This enumeration does not align with any external numbering schemes and is
/// exclusively useful within this library for requesting layer types and
/// determining which types of layers have been decoded.
/// New `LayerType`s can be created by calling `register_layer_type`.
pub type LayerTypeID = usize;

pub(crate) const MAX_LAYER_TYPE: usize = 2000;

pub enum LayerTypes {
    LayerTypeZero = 0,
    LayerTypeDecodeFailure = 1,
    LayerTypePayload = 2,
    LayerTypeFragment = 3,
}

pub struct LayerRegistry {
    decoders_by_layer_name: Mutex<HashMap<String, DecodeFunc>>,
    lt_meta_map: Mutex<HashMap<LayerTypeID, Option<LayerType>>>,
    lt_meta: [Option<LayerType>; MAX_LAYER_TYPE],
}
impl LayerRegistry {
    pub fn new() -> Self {
        LayerRegistry {
            decoders_by_layer_name: Mutex::new(HashMap::new()),
            lt_meta_map: Mutex::new(HashMap::new()),
            lt_meta: [ARRAY_REPEAT_VALUE; MAX_LAYER_TYPE], // Assuming MAX_LAYER_TYPE and LayerTypeMetadata are properly defined
        }
        //
        // let layer_type_metadata = LayerType {
        //     id: LayerTypeZero as LayerTypeID,
        //     name: "Unknown".to_owned(),
        //     decoder: Rc::from(create_decode_unknown()), // Adjust based on how decoders are implemented.
        // };
        // Self.register_layer(&layer_type_metadata, LayerTypeZero as isize)
        //     .expect("could not add layer");
        //
        // let layer_type_metadata = LayerType {
        //     id: LayerTypeDecodeFailure as LayerTypeID,
        //     name: "DecodeFailure".to_owned(),
        //     decoder: Rc::from(create_decode_unknown()), // Adjust based on how decoders are implemented.
        // };
        //
        // Self.register_layer(&layer_type_metadata, LayerTypeDecodeFailure as isize)
        //     .expect("could not add layer");
        //
        // let layer_type_metadata = LayerType {
        //     id: LayerTypePayload as LayerTypeID,
        //     name: "DecodePayload".to_owned(),
        //     decoder: Rc::from(create_decode_payload()), // Adjust based on how decoders are implemented.
        // };
        //
        // Self.register_layer(&layer_type_metadata, LayerTypePayload as isize)
        //     .expect("could not add layer");
        //
        // let layer_type_metadata = LayerType {
        //     id: LayerTypeFragment as LayerTypeID,
        //     name: "DecodeFragment".to_owned(),
        //     decoder: Rc::from(create_decode_fragment()), // Adjust based on how decoders are implemented.
        // };
        //
        // Self.register_layer(&layer_type_metadata, LayerTypeFragment as isize)
        //     .expect("could not add layer");
    }

    pub fn register_layer(&mut self, meta: &LayerType, num: isize) -> Result<(), Box<dyn Error>> {
        let n_num = num as usize;
        if 0 <= num && num < MAX_LAYER_TYPE as isize {
            let lt_meta = &self.lt_meta;
            if lt_meta[num as usize].is_some() {
                panic!("Layer type already exists");
            }
        } else {
            let mut lt_meta_map = self.lt_meta_map.lock().unwrap();
            if lt_meta_map.contains_key(&n_num) {
                let m = lt_meta_map.get_mut(&n_num);
                if m.is_some() {
                    panic!("Layer type already exists");
                }
            }
        }

        // Assuming the Decoder trait and a way to clone or reference it appropriately
        self.override_layer_type(num, meta);
        Ok(())
    }

    fn override_layer_type(&mut self, num: isize, meta: &LayerType) -> LayerTypeID {
        if 0 <= num && num < MAX_LAYER_TYPE as isize {
            // Directly override without checking if it already exists
            self.lt_meta[num as usize] = Some(meta.clone());
        } else {
            // For numbers outside the predefined range, use a map.
            // This avoids the "stupidity" comment regarding double lock by consolidating the locking operation.
            let mut lt_meta_map = self.lt_meta_map.lock().unwrap();
            lt_meta_map.insert(num as LayerTypeID, Some(meta.clone()));
        }

        // Insert or update the decoder associated with the given layer name.
        // This operation is done outside of the if-else block to avoid repetition and potential errors.
        let moved_meta = meta.clone();
        self.decoders_by_layer_name
            .lock()
            .unwrap()
            .insert(meta.name.clone(), moved_meta.decoder);

        num as LayerTypeID
    }

    // Methods to add, retrieve, and manage decoders and metadata would follow...
}
