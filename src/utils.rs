/// Modular addition (x + y = (mod n))
///
/// Overflow is thrown away by the cast down to u32.
fn z(x: u32, y: u32) -> u32 {
    (x as u64 + y as u64) as u32
}

/// Shift right by n.
fn shr(x: u32, n: u32) -> u32 {
    x >> n
}

/// Rotate right.
/// 
/// Shift bits to right, append them to lefts MSB position.
fn rotr(x: u32, n: u32) -> u32 {
    // Declare w-bit.
    let w = 32;
    // Subtract w from n to move in bit positions based on the range of n.
    (x >> n) | (x << (w - n))
}

/// Rotate left.
///
/// Simply bits to left, and append them to the right LSB position.
fn rotl(x: u32, n: u32) -> u32 {
    // Declare w-bit.
    let w = 32;
    // Subtract w from n to move in bit positions based on the range of n.
    (x << n) | (x >> (w - n))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn addition_modular_2_to_the_power_of_32_using_cast_down() {
        let x = 4294967295;
        let y = 4294967295;

        let result = z(x, y);
        let expected = 4294967294;

        assert_eq!((result), (expected));
    }

    #[test]
    fn shift_x_right_by_n() {
        let x = 312;
        let n = 2;

        let result = shr(x, n);
        let expected = 78;

        assert_eq!((result), (expected));
    }

    #[test]
    fn appends_bits_to_LSB_with_shifting_to_right() {
        let x = 15;
        let n = 3;

        let result = rotr(x, n);
        let expected = 3758096385;

        assert_eq!((result), (expected));
    }

    #[test]
    fn appends_bits_to_MSB_with_shifting_to_left() {
        let x = 3758096385;
        let n = 3;

        let result = rotl(x, n);
        let expected = 15;

        assert_eq!((result), (expected));
    }
}