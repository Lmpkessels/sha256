/// Parsing bytes.
///
/// Vector with stored bytes, which are split up into 32-bit
/// words, then stored into a block [32; 16].
/// 
/// Create four bytes, increment after each loop the byte by one.
/// Shift left backward so that MSB of byte is stored at MSB of word.
///
/// Parse entire message into a block of 16; 32-bit words.
/// Expand into next block if > 16, 32-bit words.
pub fn pars(bytes: Vec<u8>) -> Vec<[u32; 16]> {
    
    let mut words: Vec<u32> = Vec::new();
    let mut j = 0;
    // Loop as long as the message is in bytes.
    while j < bytes.len() {
        let b0 = bytes[j] as u32;
        let b1 = bytes[j + 1] as u32;
        let b2 = bytes[j + 2] as u32;
        let b3 = bytes[j + 3] as u32;

        let word = (b0 << 24) | (b1 << 16) | (b2 << 8) | (b3);

        words.push(word);
    
        // += 4 to work with 4 bytes.
        j += 4;
    } 

    let mut blocks: Vec<[u32; 16]> = Vec::new();
    let mut k = 0;
    // Loop as long as the message is in words.
    while k < words.len() {
        let mut block = [0u32; 16];
        let mut l = 0;
        while l < 16 {
            block[l] = words[k + l];
            // += 1 to work in word range.
            l += 1
        }
        blocks.push(block);
        // += 16 to work in block range. 
        k += 16;
    }

    blocks
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sha256::padd;

    #[test] 
    fn pads_message_to_16_32_bit_words() {
        let bytes = padd(b"abc"); 
        let result = pars(bytes); 

        let expected_block: [u32; 16] = [
            0x61626380, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000018
        ];
        let expected: Vec<[u32; 16]> = vec![expected_block];

        assert_eq!((result), (expected));
    }

    #[test]
    fn pads_message_to_16_32_bit_words_and_expands_into_second_block() {
        let bytes = padd(b"AAAAA_AAAAA_AAAAA_AAAAA_AAAAA_AAAAA_\
        AAAAA_AAAAA_AAAAA_AAAAA_AAAAA");
        let result = pars(bytes);

        // First 512-bit block
        let first_block: [u32; 16] = [
            1094795585, 1096761665, 1094795615, 1094795585, 
            1096761665, 1094795615, 1094795585, 1096761665, 
            1094795615, 1094795585, 1096761665, 1094795615, 
            1094795585, 1096761665, 1094795615, 1094795585
        ];

        // Second 512-bit block (from padding + length)
        let second_block: [u32; 16] = [
            1098907648, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 520
        ];

        let expected: Vec<[u32; 16]> = vec![first_block, second_block];

        assert_eq!(result, expected);
    }

}
