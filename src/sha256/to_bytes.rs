pub fn to_bytes(digest: [u32; 8]) -> [u8; 32] {
    let mut outp = [0u8; 32];
    let mut i = 0;

    while i < 8 {
        let word = digest[i];
        let j = i * 4;

        outp[j] =     (word >> 24) as u8;
        outp[j + 1] = (word >> 16) as u8;
        outp[j + 2] = (word >> 8)  as u8;
        outp[j + 3] = (word)       as u8;

        i += 1;
    }

    outp
}