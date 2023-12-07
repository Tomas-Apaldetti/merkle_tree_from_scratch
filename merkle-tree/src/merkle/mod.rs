use core::panic;
use std::{rc::Rc, marker::PhantomData};

use crate::{hashers::{Hashable, CryptoHash, sha256::SHA256, CryptoHasher}, encoding::{Digestable, hex::Hex}};

struct Node {
    hash: CryptoHash,
    right: Option<Rc<Node>>,
    left: Option<Rc<Node>>,
}

pub struct MerkleTree<T: Hashable>{
    root: Rc<Node>,
    src: PhantomData<T>
}

pub enum TreeShape{
    FullCopyExtend,
    FullNullExtend,
    PartialCopyExtend,
    PartialNullExtend,
}

impl<T: Hashable> MerkleTree<T>{
    pub fn from_data(data: &[T], tree_shape: TreeShape) -> Self{
        match tree_shape {
            TreeShape::FullCopyExtend | TreeShape::FullNullExtend => {
                Self::from_data_full_extend(data, tree_shape)
            },
            TreeShape::PartialCopyExtend | TreeShape::PartialNullExtend => {
                Self::from_data_partial_extend(data, tree_shape)
            },
            
        }
    }

    fn from_data_full_extend(data: &[T], tree_shape: TreeShape) -> Self{
        let base_nodes: Vec<Rc<Node>> = Self::extend(tree_shape,Self::nodes_from_data(&data))
            .into_iter()
            .map(|node| Rc::new(node))
            .collect();
        let root = Self::make_full_tree(&base_nodes);
        Self { root, src: PhantomData }
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
                TreeShape::FullCopyExtend => {
                    let dup = nodes[nodes.len() - original_len].hash.clone();
                    nodes.push(Node{ hash:dup, right: None, left: None});
                },
                TreeShape::FullNullExtend => {
                    nodes.push(Node { hash: null_hash.clone(), right: None, left: None })
                },
                _ => panic!("Not an extension")
            }
        }

        nodes
    }

    fn make_full_tree(nodes: &[Rc<Node>]) -> Rc<Node>{
        if nodes.len() == 1{
            return nodes[0].clone();
        }


        let half = nodes.len().next_power_of_two() / 2;
        //Right tree will always be complete 
        let left = Self::make_full_tree(&nodes[..half]);
        let right = Self::make_full_tree(&nodes[half..]);
        return Rc::new(
            Node { 
                hash: (left.hash.digest::<Hex>() + &right.hash.digest::<Hex>()).hash::<SHA256>(), 
                right: Some(right), 
                left: Some(left) 
            }
        );
    }

    fn from_data_partial_extend(data: &[T], tree_shape: TreeShape) -> Self{
        let base_nodes: Vec<Rc<Node>> = Self::nodes_from_data(&data)
            .into_iter()
            .map(|node| Rc::new(node))
            .collect();
        let filler = match tree_shape{
            TreeShape::PartialCopyExtend => None,
            TreeShape::PartialNullExtend => Some(SHA256::hash(&[0u8;256])),
            _ => panic!("Not a partial extend")
        };
        let depth: usize = f64::log2(base_nodes.len().next_power_of_two() as f64).floor() as usize;
        let (_, root) = Self::make_partial_tree(&base_nodes, depth, filler);
        Self { root, src: PhantomData }
    }

    fn make_partial_tree(nodes: &[Rc<Node>], depth: usize, filler: Option<CryptoHash>) -> (usize, Rc<Node>){
        if depth == 0{
            return (1, nodes[0].clone());
        }
        let (offset, right) = Self::make_partial_tree(nodes, depth - 1, filler.clone());
        // If when building the right I used all the nodes, then start duplicating
        if offset >= nodes.len(){
            let left = match filler{
                Some(f) => Rc::new(Node { hash: f.clone(), right: None, left: None }),
                None => Rc::new(Node { hash: right.hash.clone(), right: None, left: None }),
            };
            return (offset, Rc::new(
                Node { 
                    hash: (left.hash.digest::<Hex>() + &right.hash.digest::<Hex>()).hash::<SHA256>(), 
                    right: Some(right), 
                    left: Some(left) 
                }
            ));
        }

        //Else build the left with what is left
        let (more_offset, left) = Self::make_partial_tree(&nodes[offset..], depth - 1,filler);
        return (offset + more_offset, Rc::new(
            Node { 
                hash: (left.hash.digest::<Hex>() + &right.hash.digest::<Hex>()).hash::<SHA256>(), 
                right: Some(right), 
                left: Some(left) 
            }
        ));
    }
}