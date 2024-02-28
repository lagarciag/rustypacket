use crate::rtpacket::decode::decodefeedback::DecodeFeedback;

// A no-op implementation of DecodeFeedback.
#[derive(Default, Debug, Clone, Copy)]
pub struct NilDecodeFeedback;

impl DecodeFeedback for NilDecodeFeedback {
    fn set_truncated(&mut self) {
        // Do nothing
    }
}

// Since we're defining a static item, it needs to be `const` or `static`.
// Rust's const evaluation allows us to directly instantiate it because it's stateless and compile-time defined.
static NIL_DECODE_FEEDBACK: NilDecodeFeedback = NilDecodeFeedback;
