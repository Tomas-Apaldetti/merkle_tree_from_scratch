use crate::hashers::{Hashable, CryptoHash};

struct Node {
    right: CryptoHash,
    left: CryptoHash,
}

pub struct MerkleTree<T: Hashable>{
    root: Node,
    src: Vec<T>
}
