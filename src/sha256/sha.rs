use crate::sha256::{pars, padd, sched, compress, to_bytes};

/// SHA-256: Pads, parses, schedules, and compresses a message into a 256-bit 
/// digest.
///
/// # Argument:
/// `msg` Input message as a string slice.
///
/// # Description:
/// - Padding: Appends a single '1' bit, followed by `k` zero bits so that
///   the total length ≡ 448 (mod 512). Finally appends the original message
///   length as a 64-bit big-endian integer.
/// - Parsing: Splits the padded message into 512-bit blocks, each containing
///   sixteen 32-bit words.
/// - Scheduling: Expands each 512-bit block into a 64-word message schedule
///   using the SHA-256 Small_sigma0 and Small_sigma1 functions.
/// - Compression: Step-by-step updates the hash state across all blocks
///   to compute the final digest.
///
/// # Return
/// A 64-character lowercase hexadecimal string representing the 256-bit hash.
///
/// # Notes
/// SHA-256 supports input messages up to `2^64 − 1` bits (≈ 2.3 exabytes).
pub fn sha256(msg: &[u8]) -> [u8; 32] {
    let padding = padd(msg);
    let parsing = pars(padding);
    let schedules = sched(parsing);
    let digest = compress(schedules);
    let bytes = to_bytes(digest);

    bytes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_string_and_digest() {
        let msg = b"";
        let result = sha256(msg);
        let expected = [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 
            0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9, 0x24, 
            0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 
            0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55,
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn use_one_word_and_digest() {
        let msg = b"abc";
        let result = sha256(msg);
        let expected = [
            0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 
            0x41, 0x41, 0x40, 0xde, 0x5d, 0xae, 0x22, 0x23,
            0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 
            0xb4, 0x10, 0xff, 0x61, 0xf2, 0x00, 0x15, 0xad,
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn expanding_in_second_block_and_digest() {
        let msg = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        let result = sha256(msg);
        let expected = [
            0x24, 0x8d, 0x6a, 0x61, 0xd2, 0x06, 0x38, 0xb8, 
            0xe5, 0xc0, 0x26, 0x93, 0x0c, 0x3e, 0x60, 0x39, 
            0xa3, 0x3c, 0xe4, 0x59, 0x64, 0xff, 0x21, 0x67, 
            0xf6, 0xec, 0xed, 0xd4, 0x19, 0xdb, 0x06, 0xc1,
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn test_one_million_a_and_digest() {
        let msg = b"a".repeat(1_000_000);
        let result = sha256(&msg);
        let expected = [
            0xcd, 0xc7, 0x6e, 0x5c, 0x99, 0x14, 0xfb, 0x92, 
            0x81, 0xa1, 0xc7, 0xe2, 0x84, 0xd7, 0x3e, 0x67, 
            0xf1, 0x80, 0x9a, 0x48, 0xa4, 0x97, 0x20, 0x0e, 
            0x04, 0x6d, 0x39, 0xcc, 0xc7, 0x11, 0x2c, 0xd0,
        ];

        assert_eq!((result), (expected));
    }
}