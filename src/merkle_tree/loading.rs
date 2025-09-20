use crate::sha256::sha256;

// Create leaf loading (Check to the remainder, increment if odd, and push, 
// then return vector with hashed keys).
pub fn leaf_loading(leaf: &[[u8; 32]]) -> Vec<[u8; 32]> {
    let mut transactions = leaf.to_vec();
    
    if transactions.len() % 2 != 0 {
        // Nest into stored data to get last index.
        let last_index = transactions[transactions.len() - 1];
        transactions.push(last_index);
    }; 

    let mut leaf_nodes: Vec<[u8; 32]> = Vec::new();
    for i in &transactions {
        // Hash and store data.
        let hashed_transaction = sha256(i);    
        leaf_nodes.push(hashed_transaction);
    }

    leaf_nodes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test] 
    fn load_and_hash_leaf_then_store_them() {
        let a = [0x19u8; 32];
        let b = [0xf2u8; 32];

        let ha = sha256(&a);
        let hb = sha256(&b);

        let result = leaf_loading(&vec![a, b]);
        let expected = vec![ha, hb];

        assert_eq!((result), (expected))
    }

    #[test]
    fn load_leafs_increment_index_3_and_hash_all_values() {
        let a = [0x64u8; 32];
        let b = [0x3cu8; 32];
        let c = [0x6fu8; 32];
        
        let ha = sha256(&a);
        let hb = sha256(&b);
        let hc = sha256(&c);
        
        let result = leaf_loading(&vec![a, b, c]);
        let expected = vec![ha, hb, hc, hc];

        assert_eq!((result), (expected));
    }
}