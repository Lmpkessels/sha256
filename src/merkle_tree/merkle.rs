use crate::merkle_tree::{leaf_loading, branching};

/// Merkle tree: for efficiently verifying data.
///
/// # Arguments
/// - `leaf` - A vector of 32-byte arrays, each representing a transaction.
///
/// # Description
/// - Hash each transaction to create the leaf nodes.
/// - If the number of leaf nodes is odd, duplicate the last node.
/// - While more than one node remains:
///   - Pair nodes by index (0-1, 2-3, 4-5, etc.).
///   - Concatenate left (even index) and right (odd index).
///   - Hash the combined pair to create the parent node.
/// - Repeat until only one node remains, the -> Merkle root.
///
/// # Returns
/// - `[u8; 32]` - the Merkle root of the tree.
///
/// # References
/// - [Investopedia](https://www.investopedia.com/terms/m/merkle-tree.asp)  
/// - [Bitcoin developer guide](https://developer.bitcoin.org/devguide/block_chain.html)
fn merkle_tree(leaf: Vec<[u8; 32]>) -> [u8; 32] {
    let mut leaf_nodes = leaf_loading(&leaf);

    while leaf_nodes.len() > 1 {
        let mut non_leaf_nodes: Vec<[u8; 32]> = Vec::new();
        if leaf_nodes.len() % 2 != 0 {
            // Increment if odd.
            let last_index = leaf_nodes[leaf_nodes.len() - 1];
            leaf_nodes.push(last_index);
        };

        let mut i = 0;
        while i < leaf_nodes.len() {
            let parent_node = branching(leaf_nodes[i], leaf_nodes[i + 1]);
            non_leaf_nodes.push(parent_node);
            // Get index per 2 (0-1, 2-4, 5-6, etc).
            i = i + 2;
        }

        leaf_nodes = non_leaf_nodes;
    }

    let merkle_root = leaf_nodes[0];
    merkle_root
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sha256::sha256;

    #[test]
    fn load_even_leaf_then_branch_data_and_compute_root() {
        let a = [0x5cu8; 32];
        let b = [0x36u8; 32];
        let c = [0xcdu8; 32];
        let d = [0xa3u8; 32];
        let e = [0x94u8; 32];
        let f = [0x30u8; 32];
        let g = [0x08u8; 32];
        let h = [0x23u8; 32];

        let ha = sha256(&a);
        let hb = sha256(&b);
        let hc = sha256(&c);
        let hd = sha256(&d);
        let he = sha256(&e);
        let hf = sha256(&f);
        let hg = sha256(&g);
        let hh = sha256(&h);

        let hab = sha256(&[ha, hb].concat());
        let hcd = sha256(&[hc, hd].concat());
        let hef = sha256(&[he, hf].concat());
        let hgh = sha256(&[hg, hh].concat());

        let habcd = sha256(&[hab, hcd].concat());
        let hefgh = sha256(&[hef, hgh].concat());

        let result = merkle_tree(vec![a, b, c, d, e, f, g, h]);
        let expected = sha256(&[habcd, hefgh].concat());

        assert_eq!((result), (expected));
    }

    #[test]
    fn load_odd_leaf_then_branch_data_and_compute_root() {
        let a = [0x94u8; 32];
        let b = [0x08u8; 32];
        let c = [0x30u8; 32];
        let d = [0x5cu8; 32];
        let e = [0x9fu8; 32];
        let f = [0xffu8; 32];
        let g = [0x23u8; 32];
        
        let ha = sha256(&a);
        let hb = sha256(&b);
        let hc = sha256(&c);
        let hd = sha256(&d);
        let he = sha256(&e);
        let hf = sha256(&f);
        let hg = sha256(&g);

        let hab = sha256(&[ha, hb].concat());
        let hcd = sha256(&[hc, hd].concat());
        let hef = sha256(&[he, hf].concat());
        let hgg = sha256(&[hg, hg].concat());

        let habcd = sha256(&[hab, hcd].concat());
        let hefgh = sha256(&[hef, hgg].concat());

        let result = merkle_tree(vec![a, b, c, d, e, f, g]);
        let expected = sha256(&[habcd, hefgh].concat());

        assert_eq!((result), (expected));
    }
}