use crate::rtpacket::decode::DecodeFunc;
use crate::rtpacket::packet::decodeoptions::DecodeOptions;
use crate::rtpacket::packet::packetdatasource::PacketDataSource;

pub struct PacketSource {
    zero_copy: bool,
    source: Box<dyn PacketDataSource>,
    decoder: DecodeFunc,
    // Assuming DecodeOptions is a struct defined elsewhere
    decode_options: DecodeOptions,
}
