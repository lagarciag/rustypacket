
/// Provides a mechanism for layers to provide feedback about the decoding process.
///
/// This trait is intended to be implemented by types that can receive decoding
/// metadata, such as whether a packet has been truncated. Truncation can occur
/// if the packet's actual length is shorter than what its internal structures
/// (like headers) indicate. Implementors of this trait can use this feedback
/// to adjust their behavior or record the truncation status.
pub trait DecodeFeedback {
    /// Marks the packet as truncated.
    ///
    /// This method should be called during the decoding process if it is
    /// discovered that the packet is shorter than expected based on its
    /// internal structures, such as header lengths. Marking a packet as
    /// truncated can be important for correct interpretation of the packet's
    /// data or for diagnostic purposes.
    ///
    /// Implementations of this method should ensure that the truncation
    /// status is recorded in a way that is accessible to users of the decoded
    /// packet.
    fn set_truncated(&mut self);
}