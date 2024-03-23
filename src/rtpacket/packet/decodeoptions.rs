/// `DecodeOptions` configures how to decode a packet.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecodeOptions {
    /// Lazy decoding decodes the minimum number of layers needed to return data
    /// for a packet at each function call. Be careful using this with concurrent
    /// packet processors, as each call to packet.* could mutate the packet, and
    /// two concurrent function calls could interact poorly.
    pub lazy: bool,
    /// NoCopy decoding doesn't copy its input buffer into storage that's owned by
    /// the packet. If you can guarantee that the bytes underlying the slice
    /// passed into `new_packet` aren't going to be modified, this can be faster. If
    /// there's any chance that those bytes WILL be changed, this will invalidate
    /// your packets.
    pub no_copy: bool,
    /// Pool decoding only applies if NoCopy is false.
    /// Instead of always allocating new memory it takes the memory from a pool.
    /// `new_packet` then will return a `PooledPacket` instead of a `Packet`.
    /// As soon as you're done with the `PooledPacket` you should call `dispose` to return it to the pool.
    pub pool: bool,
    /// SkipDecodeRecovery skips over panic recovery during packet decoding.
    /// Normally, when packets decode, if a panic occurs, that panic is captured
    /// by a recover(), and a `DecodeFailure` layer is added to the packet detailing
    /// the issue. If this flag is set, panics are instead allowed to continue up
    /// the stack.
    pub skip_decode_recovery: bool,
    /// DecodeStreamsAsDatagrams enables routing of application-level layers in the TCP
    /// decoder. If true, we should try to decode layers after TCP in single packets.
    /// This is disabled by default because the reassembly package drives the decoding
    /// of TCP payload data after reassembly.
    pub decode_streams_as_datagrams: bool,
}

// Usage:
// let default_options = DecodeOptions::default();
// let lazy_options = DecodeOptions::lazy();
// let no_copy_options = DecodeOptions::no_copy();
// let datagram_options = DecodeOptions::decode_streams_as_datagrams();
impl DecodeOptions {
    pub const DEFAULT: DecodeOptions = DecodeOptions {
        lazy: false,
        no_copy: false,
        pool: false,
        skip_decode_recovery: false,
        decode_streams_as_datagrams: false,
    };

    /// Provides the default DecodeOptions, which is the safest but slowest configuration.
    pub fn default() -> Self {
        DecodeOptions {
            lazy: false,
            no_copy: false,
            pool: false,
            skip_decode_recovery: false,
            decode_streams_as_datagrams: false,
        }
    }

    /// Provides a DecodeOptions configuration with lazy decoding enabled.
    pub fn lazy() -> Self {
        DecodeOptions {
            lazy: true,
            ..Self::default()
        }
    }

    /// Provides a DecodeOptions configuration with no_copy decoding enabled.
    pub fn no_copy() -> Self {
        DecodeOptions {
            no_copy: true,
            ..Self::default()
        }
    }

    /// Provides a DecodeOptions configuration with decode_streams_as_datagrams enabled.
    pub fn decode_streams_as_datagrams() -> Self {
        DecodeOptions {
            decode_streams_as_datagrams: true,
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let options = DecodeOptions::default();
        assert!(!options.lazy, "Default should not be lazy.");
        assert!(!options.no_copy, "Default should not be no_copy.");
        assert!(!options.pool, "Default should not use pool.");
        assert!(
            !options.skip_decode_recovery,
            "Default should not skip decode recovery."
        );
        assert!(
            !options.decode_streams_as_datagrams,
            "Default should not decode streams as datagrams."
        );
    }

    #[test]
    fn test_lazy_options() {
        let options = DecodeOptions::lazy();
        assert!(options.lazy, "Lazy options should be lazy.");
        assert!(
            !options.no_copy,
            "Lazy options should not be no_copy by default."
        );
        assert!(
            !options.pool,
            "Lazy options should not use pool by default."
        );
        assert!(
            !options.skip_decode_recovery,
            "Lazy options should not skip decode recovery by default."
        );
        assert!(
            !options.decode_streams_as_datagrams,
            "Lazy options should not decode streams as datagrams by default."
        );
    }

    #[test]
    fn test_no_copy_options() {
        let options = DecodeOptions::no_copy();
        assert!(
            !options.lazy,
            "No-copy options should not be lazy by default."
        );
        assert!(options.no_copy, "No-copy options should be no_copy.");
        assert!(
            !options.pool,
            "No-copy options should not use pool by default."
        );
        assert!(
            !options.skip_decode_recovery,
            "No-copy options should not skip decode recovery by default."
        );
        assert!(
            !options.decode_streams_as_datagrams,
            "No-copy options should not decode streams as datagrams by default."
        );
    }

    #[test]
    fn test_decode_streams_as_datagrams_options() {
        let options = DecodeOptions::decode_streams_as_datagrams();
        assert!(
            !options.lazy,
            "Decode streams as datagrams options should not be lazy by default."
        );
        assert!(
            !options.no_copy,
            "Decode streams as datagrams options should not be no_copy by default."
        );
        assert!(
            !options.pool,
            "Decode streams as datagrams options should not use pool by default."
        );
        assert!(
            !options.skip_decode_recovery,
            "Decode streams as datagrams options should not skip decode recovery by default."
        );
        assert!(
            options.decode_streams_as_datagrams,
            "Decode streams as datagrams options should decode streams as datagrams."
        );
    }
}
