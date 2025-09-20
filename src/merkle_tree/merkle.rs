use crate::merkle_tree::{leaf_loading, branching};

// Create merkle-tree (leaf-loading, compression, resulted key from appended
// keys).
fn merkle_tree(leaf: Vec<[u8; 32]>) -> [u8; 32] {
    let mut stored_data = leaf_loading(&leaf);

    while stored_data.len() > 1 {
        let mut root: Vec<[u8; 32]> = Vec::new();
        if stored_data.len() % 2 != 0 {
            let last_index = stored_data[stored_data.len() - 1];
            stored_data.push(last_index);
        };

        let mut i = 0;
        while i < stored_data.len() {
            let parent = branching(stored_data[i], stored_data[i + 1]);
            root.push(parent);
            // Get index per 2 (0-1, 2-4, 5-6, etc).
            i = i + 2;
        }

        stored_data = root;
    }

    stored_data[0]
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