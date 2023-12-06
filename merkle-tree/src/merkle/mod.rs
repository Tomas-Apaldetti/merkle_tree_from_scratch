use core::panic;
use std::rc::Rc;

use crate::{hashers::{Hashable, CryptoHash, sha256::SHA256, CryptoHasher}, encoding::{Digestable, hex::Hex}};

struct Node {
    hash: CryptoHash,
    right: Option<Rc<Node>>,
    left: Option<Rc<Node>>,
}

pub struct MerkleTree<T: Hashable>{
    root: Rc<Node>,
    src: Vec<T>
}

pub enum TreeShape{
    CopyExtend,
    NullExtend,
}

impl<T: Hashable> MerkleTree<T>{
    pub fn from_data(data: Vec<T>, tree_shape: TreeShape) -> Self{
        let base_nodes: Vec<Rc<Node>> = Self::extend(tree_shape, Self::nodes_from_data(&data))
            .into_iter()
            .map(|node| Rc::new(node))
            .collect();
        let root = Self::make_tree(&base_nodes);
        Self { root, src: data }
    }

    fn nodes_from_data(data: &[T]) -> Vec<Node>{
        let mut nodes = Vec::with_capacity(data.len().next_power_of_two());
        for datoid in data{
            nodes.push(Node { hash: datoid.hash::<SHA256>(), right: None, left: None })
        }

        nodes
    }

    fn extend(extend_type: TreeShape, mut nodes:Vec<Node>) -> Vec<Node>{
        let original_len = nodes.len();
        let extend_to = nodes.len().next_power_of_two();
        let null_hash = SHA256::hash(&[0u8;256]);
        while nodes.len() != extend_to{
            match extend_type{
                TreeShape::CopyExtend => {
                    let dup = nodes[nodes.len() - original_len].hash.clone();
                    nodes.push(Node{ hash:dup, right: None, left: None});
                },
                TreeShape::NullExtend => {
                    nodes.push(Node { hash: null_hash.clone(), right: None, left: None })
                },
                _ => panic!("Not an extension")
            }
        }

        nodes
    }

    fn make_tree(nodes: &[Rc<Node>]) -> Rc<Node>{
        if nodes.len() == 1{
            return nodes[0].clone();
        }
        let half = nodes.len().next_power_of_two() / 2;
        //Right tree will always be complete 
        let left = Self::make_tree(&nodes[..half]);
        let right = Self::make_tree(&nodes[half..]);
        return Rc::new(
            Node { 
                hash: (left.hash.digest::<Hex>() + &right.hash.digest::<Hex>()).hash::<SHA256>(), 
                right: Some(right), 
                left: Some(left) 
            }
        );
    }

}