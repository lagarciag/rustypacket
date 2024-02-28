use std::error::Error;
use crate::rtpacket::capture::CaptureInfo;

pub trait PacketDataSource {
    // ReadPacketData attempts to read the next packet from the data source.
    // On success, it returns a tuple of the packet data as a Vec<u8> and capture info.
    // On failure, it returns an error.
    fn read_packet_data(&self) -> Result<(Vec<u8>, CaptureInfo), Box<dyn Error>>;
}

pub struct Concat(Vec<Box<dyn PacketDataSource>>);


impl Concat {
    pub fn read_packet_data(&mut self) -> Result<(Vec<u8>, CaptureInfo), Box<dyn Error>> {
        while !self.0.is_empty() {
            match self.0[0].read_packet_data() {
                Ok(data) => return Ok(data),
                Err(e) => {
                    if e.downcast_ref::<std::io::Error>().map_or(false, |err| err.kind() == std::io::ErrorKind::UnexpectedEof) {
                        self.0.remove(0); // Remove the first element and continue if EOF
                        continue;
                    }
                    return Err(e);
                },
            }
        }
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "EOF")))
    }
}

