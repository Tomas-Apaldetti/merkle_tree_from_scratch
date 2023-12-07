use std::{rc::Rc, marker::PhantomData};

use crate::{hashers::{Hashable, CryptoHasher, CryptoHash}, encoding::{Digestable, Digester}};

use super::{node::Node, TreeShape, merkle_trace::MerkleTrace};

pub struct MerkleTree<T: Hashable>{
    root: Rc<Node>,
    original_len: usize,
    src: PhantomData<T>
}

impl<T: Hashable> MerkleTree<T>{
    pub fn from_data<H: CryptoHasher, D:Digester>(data: &[T], tree_shape: TreeShape) -> Self{
        let filler = match tree_shape{
            TreeShape::PartialNullExtend => Some(H::hash(&[0u8;256])),
            _ => None
        };

        let nodes: Vec<Rc<Node>> = match tree_shape {
            TreeShape::FullCopyExtend | TreeShape::FullNullExtend => {
                Self::extend::<H>(
                    Self::nodes_from_data::<H>(&data), 
                    tree_shape
                )
                .into_iter()
                .map(|node| Rc::new(node))
                .collect()
            },
            TreeShape::PartialCopyExtend | TreeShape::PartialNullExtend => {
               Self::nodes_from_data::<H>(&data)
                    .into_iter()
                    .map(|node| Rc::new(node))
                    .collect()
            },   
        };

        let depth: usize = f64::log2(nodes.len().next_power_of_two() as f64).floor() as usize;

        let (_, root) = Self::make_partial_tree::<H,D>(&nodes, depth, filler);
        Self { root, original_len: data.len(), src: PhantomData }
    }

    fn nodes_from_data<H: CryptoHasher>(data: &[T]) -> Vec<Node>{
        let mut nodes = Vec::with_capacity(data.len().next_power_of_two());
        for datoid in data{
            nodes.push(Node { hash: datoid.hash::<H>(), right: None, left: None })
        }

        nodes
    }

    fn extend<H: CryptoHasher>(mut nodes:Vec<Node>, extend_type: TreeShape) -> Vec<Node>{
        let original_len = nodes.len();
        let extend_to = nodes.len().next_power_of_two();
        let null_hash = H::hash(&[0u8;256]);
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

    fn make_partial_tree<H: CryptoHasher, D: Digester>(nodes: &[Rc<Node>], depth: usize, filler: Option<CryptoHash>) -> (usize, Rc<Node>){
        if depth == 0{
            return (1, nodes[0].clone());
        }
        let (offset, right) = Self::make_partial_tree::<H,D>(nodes, depth - 1, filler.clone());
        // If when building the right I used all the nodes, then start duplicating
        if offset >= nodes.len(){
            let left = match filler{
                Some(f) => Rc::new(Node { hash: f.clone(), right: None, left: None }),
                None => Rc::new(Node { hash: right.hash.clone(), right: None, left: None }),
            };
            return (offset, Rc::new(
                Node { 
                    hash: (left.hash.digest::<D>() + &right.hash.digest::<D>()).hash::<H>(), 
                    right: Some(right), 
                    left: Some(left) 
                }
            ));
        }

        //Else build the left with what is left
        let (more_offset, left) = Self::make_partial_tree::<H,D>(&nodes[offset..], depth - 1,filler);
        return (offset + more_offset, Rc::new(
            Node { 
                hash: (left.hash.digest::<D>() + &right.hash.digest::<D>()).hash::<H>(), 
                right: Some(right), 
                left: Some(left) 
            }
        ));
    }
}

impl<T: Hashable> MerkleTree<T> {
    pub fn generate_trace(&self, which: usize) -> Result<MerkleTrace, ()>{
        if which > self.original_len {
            return Err(())
        }

        return Ok(self.trace(which));
    }

    fn trace(&self, which: usize) -> MerkleTrace{
        let root = Self::search(self.root.clone(), which, 0, self.original_len.next_power_of_two());

        MerkleTrace { root }
    }

    fn search(root: Rc<Node>, which: usize, left: usize, rigth: usize) -> Rc<Node>{
        if root.right.is_none() && root.left.is_none() {
            //By construction, is either of those is none, the other is also
            return Rc::new(Node{hash: root.hash.clone(), right: None, left: None});
        }
        
        // Always a power of two
        let mid = (left + rigth ) / 2;
        if which < mid  {
            let rigth = Self::search(root.right.clone().unwrap(), which, mid, rigth);
            let left = Rc::new(Node {hash:root.left.clone().unwrap().hash.clone(), right: None, left: None});
            return Rc::new(Node { hash: root.hash.clone(), right: Some(rigth), left: Some(left) })
        }

        let rigth = Rc::new(Node {hash:root.right.clone().unwrap().hash.clone(), right: None, left: None});
        let left = Self::search(root.left.clone().unwrap(), which, left, mid);
        return Rc::new(Node { hash: root.hash.clone(), right: Some(rigth), left: Some(left) });
    }
}