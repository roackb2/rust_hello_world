use super::{
  transaction::Transaction,
  node::*
};

#[derive(Debug)]
pub struct MerkleTree {
  root: Box<InternalNode>,
}

impl MerkleTree {
  pub fn new(data: Transaction) -> MerkleTree {
    let leaf = Node::Leaf(Box::new(LeafNode::new(data)));
    let mut node = InternalNode::new();
    node.append(leaf);
    MerkleTree {
      root: Box::new(node),
    }
  }
  pub fn insert(&mut self, txn: Transaction) -> bool {
    (*self.root).insert(txn)
  }
}
