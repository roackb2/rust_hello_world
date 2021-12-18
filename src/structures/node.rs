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
  pub fn insert(&mut self, data: Transaction) -> bool {
    
    false
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
  pub fn append(&mut self, node: Node) -> bool {
    if let None = self.left {
      self.left = Some(Box::new(node));
      return true;
    } else if let None = self.right {
      self.right = Some(Box::new(node));
      return true;
    } else {
      panic!("Node is full");
    }
  }
  pub fn is_full(&self) -> bool {
    if let (Some(_), Some(_)) = (&self.left, &self.right) {
      return true
    }
    false
  }
  // TODO: update hash
  pub fn insert(&mut self, txn: Transaction) -> bool {
    if !self.is_full() {
      let leaf = LeafNode::new(txn);
      return self.append(Node::Leaf(leaf));
    } else if let Some(bxl) = self.left.as_mut() {
      return (*bxl).insert(txn);
    } else if let Some(bxr) = self.right.as_mut() {
      return (*bxr).insert(txn);
    }
    false
  }
}

#[derive(Debug)]
pub enum Node {
  Leaf(LeafNode),
  Internal(InternalNode),
}

impl Node {
  pub fn insert(&mut self, txn: Transaction) -> bool {
    match self {
      Node::Leaf(node) => node.insert(txn),
      Node::Internal(node) => node.insert(txn)
    }
  }
  pub fn is_full(&self) -> bool {
    match self {
      Node::Leaf(_) => true,
      Node::Internal(node) => node.is_full()
    }
  }
}


impl Hash for Node {
  fn hash<H: Hasher>(&self, s: &mut H) {
    match self {
      Node::Leaf(trx) => trx.hash(s),
      Node::Internal(node) => node.hash(s)
    }
  }
}