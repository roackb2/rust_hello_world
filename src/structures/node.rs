use std::collections::hash_map::DefaultHasher;
use std::hash::{ Hash, Hasher };
use crate::structures::transaction::{ Transaction };
use crate::structures::utils::{ get_hash, get_empty_hash };

#[derive(Debug)]
pub struct LeafNode {
  hash: u64,
  data: Box<Transaction>,
}

impl Hash for LeafNode {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.data.hash(state);
    self.hash.hash(state);
  }
}

impl LeafNode {
  pub fn new(data: Transaction) -> LeafNode {
    let hash = get_hash(&data);
    LeafNode {
      data: Box::new(data),
      hash
    }
  }
}

#[derive(Debug)]
pub struct InternalNode {
  hash: u64,
  left: Option<Box<Node>>,
  right: Option<Box<Node>>,
}

impl Hash for InternalNode {
  fn hash<H: Hasher>(&self, s: &mut H) {
    self.left.hash(s);
    self.right.hash(s);
    self.hash.hash(s);
  }
}

pub fn get_child_hash(left: u64, right: u64) -> u64 {
  let mut state = DefaultHasher::new();
  left.hash(&mut state);
  right.hash(&mut state);
  state.finish()
}

impl InternalNode {
  pub fn new() -> InternalNode {
    InternalNode {
      left: None,
      right: None,
      hash: get_empty_hash()
    }
  }
  pub fn append(&mut self, node: Node) {
    if let None = self.left {
      self.left = Some(Box::new(node));
    } else if let None = self.right {
      self.right = Some(Box::new(node));
    } else {
      panic!("Node is full");
    }
  }
  pub fn is_full(&self) -> bool {
    if let (Some(l), Some(r)) = (&self.left, &self.right) {
      return true
    }
    false
  }
}

#[derive(Debug)]
pub enum Node {
  Leaf(LeafNode),
  Internal(InternalNode),
}

impl Hash for Node {
  fn hash<H: Hasher>(&self, s: &mut H) {
    match self {
      Node::Leaf(trx) => trx.hash(s),
      Node::Internal(node) => node.hash(s)
    }
  }
}