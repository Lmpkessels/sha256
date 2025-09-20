use crate::sha256::sha256;

// Create branch (take loaded leafts - left/right, push them into vec
// with capacity 64, hash the vec, return the hashed state).
pub fn branching(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
    let mut appended_left_right = Vec::with_capacity(64);

    for i in 0..left.len() {
        appended_left_right.push(left[i]);
    }

    for j in 0..left.len() {
        appended_left_right.push(right[j]);
    }

    sha256(&appended_left_right)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn append_hashed_leafs_1by1_and_hash_appended_computation() {
        let a = [0xccu8; 32];
        let b = [0x1du8; 32];
        
        let mut appended_leafs = Vec::with_capacity(64);
        for i in 0..a.len() {
            appended_leafs.push(a[i]);
        }

        for j in 0..b.len() {
            appended_leafs.push(b[j]);
        }

        let result = branching(a, b);
        let expected = sha256(&appended_leafs);

        assert_eq!((result), (expected));
    }
}