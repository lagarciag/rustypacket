use crate::rtpacket::base::Layer;

/// Represents the outcome of checksum verification, including both successful
/// and unsuccessful verifications.
#[derive(Debug, Clone, Copy)]
pub struct ChecksumVerificationResult {
    /// Indicates whether the checksum verification was successful.
    pub valid: bool,
    /// The correct checksum that was expected.
    pub correct: u32,
    /// The actual checksum found, which may be incorrect.
    pub actual: u32,
}

/// Provides detailed information about a failed checksum verification for a layer.
pub struct ChecksumMismatch {
    /// The checksum verification result that failed.
    pub result: ChecksumVerificationResult,
    /// The layer whose checksum verification failed.
    // Assuming `Layer` is a trait defined elsewhere.
    pub layer: Box<dyn Layer>,
    /// The index of the layer within the packet.
    pub layer_index: usize,
}

/// Computes the internet checksum as defined in RFC1071, allowing for an
/// initial checksum value to be provided.
pub fn compute_checksum(data: &[u8], mut csum: u32) -> u32 {
    let length = data.len();
    for i in (0..length).step_by(2) {
        // Combine two bytes at a time into one 16-bit number and add it to the checksum
        csum += (data[i] as u32) << 8;
        // Check if there's a next byte to add, otherwise, skip
        if i + 1 < length {
            csum += data[i + 1] as u32;
        }
    }
    // If there's an odd number of bytes, pad the last byte with zeros and add it to the checksum
    if length % 2 == 1 {
        csum += (data[length - 1] as u32) << 8;
    }

    csum
}

/// Folds a 32-bit checksum into 16 bits as defined in RFC1071, applying
/// 1's complement on the result.
pub fn fold_checksum(csum: u32) -> u16 {
    let mut csum = csum;
    while csum > 0xffff {
        csum = (csum >> 16) + (csum & 0xffff);
    }
    !(csum as u16)
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

// A helper function to fold the checksum if needed

    #[test]
    fn test_checksum() {
        let test_data = vec![
            (
                "sum has two carries",
                "4540005800000000ff11ffff0aeb1d070aed8877",
                "fffe",
            ),
            (
                "wikipedia case",
                "45000073000040004011b861c0a80001c0a800c7",
                "b861",
            ),
        ];

        for (name, header, want) in test_data {
            let bytes = hex::decode(header).expect("Failed to decode header");

            let want_bytes = hex::decode(want).expect("Failed to decode want checksum");

            let want_checksum: u16 = u16::from_be_bytes(want_bytes.try_into().unwrap());

            // Clear checksum bytes
            let mut bytes_copy = bytes.clone();
            bytes_copy[10] = 0;
            bytes_copy[11] = 0;

            let csum = compute_checksum(&bytes_copy, 0);
            let folded_csum = fold_checksum(csum);

            assert_eq!(
                folded_csum, want_checksum,
                "In test {:?}, got incorrect checksum: got({:x}), want({:x})",
                name, folded_csum, want_checksum
            );
        }
    }
}

