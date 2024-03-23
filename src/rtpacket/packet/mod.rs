use std::error::Error;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;

use hex::encode as hex_encode;
use lazy_static::lazy_static;

use crate::rtpacket::base::Layer;
use crate::rtpacket::decode::DecodeFunc;

use super::capture::CaptureInfo;
use super::packet::decodeoptions::DecodeOptions;
use super::packet::packetable::{Packet, Packetable};

pub mod decodeoptions;
mod eagerpacket;
mod packetable;
mod packetbase;
mod packetdatasource;
mod packetsource;
mod zerocopy;

const MAXIMUM_MTU: usize = 1500;

// Define a global error for "no layers added"
// In Rust, it's common to define errors using enums or structs that implement the Error trait.
// For simplicity, we'll use a static string. For a more comprehensive solution, consider using
// custom error types or the `anyhow` crate for flexible error handling.
pub const ERR_NO_LAYERS_ADDED: &str = "NextDecoder called, but no layers added yet";

// Define a simple pool to reuse byte vectors up to a certain size (maximumMTU).
// This basic example uses a Mutex to provide thread-safe access to a pool of Vec<u8>.
// A more efficient implementation might use a concurrent data structure or a more complex
// pooling mechanism.
pub struct BytePool {
    pool: Mutex<Vec<Vec<u8>>>,
    capacity: usize,
}

impl BytePool {
    pub fn new(capacity: usize) -> Self {
        BytePool {
            pool: Mutex::new(Vec::new()),
            capacity,
        }
    }

    // Get a Vec<u8> from the pool or create a new one if the pool is empty.
    pub fn get(&self) -> Vec<u8> {
        let mut pool = self.pool.lock().unwrap();
        pool.pop()
            .unwrap_or_else(|| Vec::with_capacity(self.capacity))
    }

    // Return a Vec<u8> to the pool if it's not larger than the maximum capacity.
    pub fn put(&self, mut vec: Vec<u8>) {
        if vec.capacity() <= self.capacity {
            vec.clear();
            let mut pool = self.pool.lock().unwrap();
            pool.push(vec);
        }
    }
}

lazy_static! {
    static ref POOL_PACKED_POOL: BytePool = BytePool::new(MAXIMUM_MTU); // Assuming maximumMTU is 1500
}

struct PacketSource {
    /// Lazy decoding decodes the minimum number of layers needed to return data
    /// for a packet at each function call. Be careful using this with concurrent
    /// packet processors, as each call to packet.* could mutate the packet, and
    /// two concurrent function calls could interact poorly.
    pub lazy: bool,

    /// NoCopy decoding doesn't copy its input buffer into storage that's owned by
    /// the packet. If you can guarantee that the bytes underlying the slice
    /// passed into NewPacket aren't going to be modified, this can be faster. If
    /// there's any chance that those bytes WILL be changed, this will invalidate
    /// your packets.
    pub no_copy: bool,

    /// Pool decoding only applies if NoCopy is false.
    /// Instead of always allocating new memory it takes the memory from a pool.
    /// NewPacket then will return a PooledPacket instead of a Packet.
    /// As soon as you're done with the PooledPacket you should call PooledPacket.Dispose() to return it to the pool.
    pub pool: bool,

    /// SkipDecodeRecovery skips over panic recovery during packet decoding.
    /// Normally, when packets decode, if a panic occurs, that panic is captured
    /// by a recover(), and a DecodeFailure layer is added to the packet detailing
    /// the issue. If this flag is set, panics are instead allowed to continue up
    /// the stack.
    pub skip_decode_recovery: bool,

    /// DecodeStreamsAsDatagrams enables routing of application-level layers in the TCP
    /// decoder. If true, we should try to decode layers after TCP in single packets.
    /// This is disabled by default because the reassembly package drives the decoding
    /// of TCP payload data after reassembly.
    pub decode_streams_as_datagrams: bool,
    pub zero_copy: bool,
    pub source: Box<dyn FnMut() -> Result<(Vec<u8>, CaptureInfo), Box<dyn Error>>>,
    pub decoder: DecodeFunc,
    c: Receiver<Packet>,
}

fn layer_string(layer: Box<dyn Layer>) -> String {
    // Directly using the `string` method if available
    layer.string()
}

/// `Dumper` is a trait for types that can dump verbose information about themselves.
/// If a layer type implements `Dumper`, then its `layer_dump()` method's output
/// should include the results of `dump()`. This is intended for debugging or
/// detailed logging where a concise representation of a type and its current state
/// is needed.
pub trait Dumper {
    /// Dumps verbose information about the implementing type.
    ///
    /// Returns a `String` containing the detailed information.
    fn dump(&self) -> String;
}

/// Outputs a very verbose string representation of a layer.
/// Its output is a concatenation of the layer's string representation, a newline,
/// optionally the layer's dump (if it implements Dumper), and a hex dump of the layer's contents.
/// It contains newlines and ends with a newline.
fn layer_dump(l: Rc<dyn Layer>) -> String {
    let contents = l.layer_contents().unwrap();
    hex_encode(contents.as_ref())
}

fn new_packet(
    _data: &[u8],
    _first_layer_decoder: DecodeFunc,
    _options: DecodeOptions,
) -> Box<dyn Packetable> {
    todo!()
}
