use crate::rtpacket::capture::CaptureInfo;

/// `ZeroCopyPacketDataSource` is a trait for sources that allow packet data to be returned
/// without copying it to a user-controlled buffer. It's very similar to `PacketDataSource`,
/// except that the caller must be more careful in how the returned buffer is handled.
pub trait ZeroCopyPacketDataSource {
    /// Returns the next packet available from this data source without copying the data.
    ///
    /// # Returns
    ///
    /// - `data`: The bytes of an individual packet. Unlike with `PacketDataSource`'s `read_packet_data`,
    ///   the slice returned here points to a buffer owned by the data source. In particular, the bytes in
    ///   this buffer may be changed by future calls to `zero_copy_read_packet_data`. The returned buffer
    ///   must not be used after subsequent `zero_copy_read_packet_data` calls.
    /// - `ci`: Metadata about the capture.
    /// - `err`: An error encountered while reading packet data. If `err` is not `None`,
    ///   then `data`/`ci` will be ignored.
    ///
    /// The `'a` lifetime parameter specifies that the returned data slice is valid for the lifetime `'a`.
    /// This means the data can be used as long as it does not outlive the source it comes from.
    fn zero_copy_read_packet_data(&self) -> Result<(&[u8], CaptureInfo), Box<dyn std::error::Error>>;
}
