use std::rc::Rc;

use super::node::Node;

pub struct MerkleTrace{
    pub(crate) root: Rc<Node>,
}