use std::rc::Rc;

use crate::hashers::CryptoHash;

pub(crate) struct Node {
    pub(crate) hash: CryptoHash,
    pub(crate) right: Option<Rc<Node>>,
    pub(crate) left: Option<Rc<Node>>,
}