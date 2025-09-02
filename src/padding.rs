// Get length of the MSG (l).
fn l(msg: &str) -> usize {
    let mut total_bits = 0;

    for c in msg.chars() {
        let byte_length = c.len_utf8();
        // Multiply bit length by 8 bits.
        // Here i could've asked how many bits does a byte have since that's
        // the default value of all words etc, a word of 32 bits is an
        // array of 4 * a byte.
        let bit_length = byte_length * 8;
        total_bits += bit_length;
    }

    total_bits
}   

// Append 0s to message (k).
fn append_0s(msg: usize) -> usize {
    let mut result = 0;

    // Now were working with the integer 512 which is 10000_00000, that's not
    // 512 bits, 512 bits i 1 << 512;
    if msg < 448 {
        result = 448 - (msg + 1);
    }

    //TODO: Handle > 512.

    result
}

// Pad it all together (l + 1 + k + m-bit).
fn padding(msg: &str) -> usize {
    let length = l(msg);

    // Padding (l + 1 + k + m-bit).
    // Since this isn't working at the bit level yet append default
    // word width of the message.
    length + 1 + append_0s(length) + 64
}

fn main() {
    let test = l("abc");
    println!("{test:?}");

    let test0 = padding("Hello, reader of this file.");
    println!("{test0:?}");
}