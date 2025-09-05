/// Pads the given message with 0x80 and zeroes until its length is congruent to 56 mod 64.
/// This prepares the message for appending the 64-bit length field in the next step.
fn count_length(msg: &str) -> Vec<u8> {
    // Compute byte length of message.
    let bytes: &[u8] = msg.as_bytes();

    // Compute original length in bits.
    //
    // Multiply the original length times 8 to work with 8-bit range (byte).
    let l_bits: u64 = (bytes.len() as u64) * 8;

    // Start mutable buffer.
    // Move bytes into vec so it can grow exponentially (heap allocated).
    let mut v: Vec<u8> = bytes.to_vec();

    // Append; end of message bit.
    //
    // Note: If the message is 56 then total length is 57 since we append 1 (0x80), 
    // so the original message can be 55 characters. 
    // Else an entire new block is created filled with 0s and the msg length.
    v.push(0x80);

    // Loop for as long as the message doesn't contain 56 bytes.
    // While looping till 56 bytes, append 0s (k).
    // 
    // If msg > 56 (including + 0x80) new block is created.
    while v.len() % 64 != 56 {
        v.push(0x00);
    }

    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn computes_msg_length_appends_0x80_and_k_till_56_bytes_in_length() {
        let msg = "AAAAAAAAAA_AAAAAAAAAA_AAAAAAAAAA_AAAAAAAAAA_AAAAA";
        let result = count_length(msg);
        let expected = [
            65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 95,
            65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 95, 
            65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 95, 
            65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 95, 
            65, 65, 65, 65, 65, 128, 0, 0, 0, 0, 0, 0
        ];
        
        assert_eq!((result), (expected));
    }
}