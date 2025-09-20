use crate::sha256::sha256;

/// Merkle-tree branching: combine left and right nodes into a parent node.
///
/// # Arguments
/// - `left_node` - 32-byte array representing the left child.
/// - `right_node` - 32-byte array representing the right child.
///
/// # Description
/// - Append all bytes from the left node into a 64-byte vector.
/// - Append all bytes from the right node into the same vector.
/// - Hash the combined 64 bytes to create the parent node.
///
/// # Returns
/// - `[u8; 32]` - the parent node hash.
pub fn branching(left_node: [u8; 32], right_node: [u8; 32]) -> [u8; 32] {
    let mut combined = Vec::with_capacity(64);

    for i in 0..left_node.len() {
        combined.push(left_node[i]);
    }

    for j in 0..right_node.len() {
        combined.push(right_node[j]);
    }

    sha256(&combined)
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