use std::rc::Rc;

use crate::{hashers::{Hashable, CryptoHash, sha256::SHA256}, encoding::{Digestable, hex::Hex}};

struct Node {
    hash: CryptoHash,
    right: Option<Rc<Node>>,
    left: Option<Rc<Node>>,
}

pub struct MerkleTree<T: Hashable>{
    root: Rc<Node>,
    src: Vec<T>
}


impl<T: Hashable> MerkleTree<T>{
    pub fn from_data(data: Vec<T>) -> Self{
        let base_nodes: Vec<Rc<Node>> = Self::nodes_from_data(&data).into_iter().map(|node| Rc::new(node)).collect();
        let root = Self::make_tree(&base_nodes);
        Self { root, src: data }
    }

    fn nodes_from_data(data: &[T]) -> Vec<Node>{
        let mut nodes = Vec::with_capacity(data.len().next_power_of_two());
        for datoid in data{
            nodes.push(Node { hash: datoid.hash::<SHA256>(), right: None, left: None })
        }
        let mut i = 0;
        while nodes.len() != data.len().next_power_of_two() {
            let dup = nodes[i].hash.clone();
            nodes.push(Node{ hash:dup, right: None, left: None});
            i += 1;
        }

        nodes
    }

    fn make_tree(nodes: &[Rc<Node>]) -> Rc<Node>{
        if nodes.len() == 1{
            return nodes[0].clone();
        }
        let half = nodes.len().next_power_of_two() / 2;
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