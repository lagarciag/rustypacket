use std::any::Any;
use std::rc::Rc;
use std::time::SystemTime;

pub trait AnyClone: Any {
    fn clone_box(&self) -> Box<dyn AnyClone>;
}

fn clone_any_clone_box(item: &Box<dyn AnyClone>) -> Box<dyn AnyClone> {
    item.clone_box()
}

#[macro_export]
macro_rules! define_capture_info_struct {
    ($name:ident { $($field_name:ident: $field_type:ty),* $(,)* }) => {

        pub struct $name {
            /// Timestamp is the time the packet was captured, if that is known.
            pub timestamp: SystemTime,
            /// CaptureLength is the total number of bytes read off of the wire.
            pub capture_length: usize,
            /// Length is the size of the original packet. Should always be >=
            /// CaptureLength.
            pub length: usize,
            /// InterfaceIndex identifies the network interface from which the packet
            /// was captured, if applicable.
            pub interface_index: usize,
            /// The packet source can place ancillary data of various types here.
            /// For example, a packet capture source might report the VLAN of captured
            /// packets this way.
            pub ancillary_data: Vec<Box<dyn AnyClone>>,

            // Additional fields specified by the user
            $(pub $field_name: $field_type),*
        }
        impl AnyClone for $name {
            fn clone_box(&self) -> Box<dyn AnyClone> {
                Box::new(self.clone())
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                $name {
                    timestamp: self.timestamp,
                    capture_length: self.capture_length,
                    length: self.length,
                    interface_index: self.interface_index,
                    ancillary_data: self.ancillary_data.iter().map(clone_any_clone_box).collect(),
                    $($field_name: self.$field_name.clone()),*
                }
            }
        }
    };
}

// CaptureInfo provides standardized information about a packet captured off
// the wire or read from a file.
pub struct CaptureInfo {
    /// Timestamp is the time the packet was captured, if that is known.
    pub timestamp: SystemTime,
    /// CaptureLength is the total number of bytes read off of the wire.
    pub capture_length: usize,
    /// Length is the size of the original packet. Should always be >=
    /// CaptureLength.
    pub length: usize,
    /// InterfaceIndex identifies the network interface from which the packet
    /// was captured, if applicable.
    pub interface_index: usize,
    /// The packet source can place ancillary data of various types here.
    /// For example, a packet capture source might report the VLAN of captured
    /// packets this way.
    pub ancillary_data: Vec<Box<dyn AnyClone>>,
}

impl CaptureInfo {
    /// Adds ancillary data to the `CaptureInfo`. The data must be boxed since `dyn Any` is a trait object.
    pub fn add_ancillary_data(&mut self, data: Box<dyn AnyClone>) {
        self.ancillary_data.push(data);
    }
}

// Contains metadata for a packet, including capture information and
// a flag indicating if the packet data is truncated.
#[derive(Clone)]
pub struct PacketMetadata {
    /// Timestamp is the time the packet was captured, if that is known.
    pub timestamp: SystemTime,

    /// CaptureLength is the total number of bytes read off of the wire.
    pub capture_length: usize,

    /// Length is the size of the original packet. Should always be >=
    /// CaptureLength.
    pub length: usize,

    /// InterfaceIndex identifies the network interface from which the packet
    /// was captured, if applicable.
    pub interface_index: usize,

    /// The packet source can place ancillary data of various types here.
    /// For example, a packet capture source might report the VLAN of captured
    /// packets this way.
    pub ancillary_data: Vec<Rc<dyn AnyClone>>,

    /// Indicates whether the packet's data is truncated compared to what
    /// the packet's headers indicate. This can happen due to errors in
    /// packet formation or due to partial capture of the packet data.
    pub truncated: bool,
}
