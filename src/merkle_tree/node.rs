use std::collections::hash_map::DefaultHasher;
use std::hash::{ Hash, Hasher };
use super::{
  utils,
  transaction::Transaction,
};

type TraverseCb = fn(&Node) -> bool;

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
    let hash = utils::get_hash(&data);
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
  left: Node,
  right: Node,
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
      left: Node::None,
      right: Node::None,
      hash: utils::get_empty_hash()
    }
  }
  pub fn append(&mut self, node: Node) -> bool {
    if let Node::None = self.left {
      self.left = node;
      return true;
    } else if let Node::None = self.right {
      self.right = node;
      return true;
    } else {
      panic!("Node is full");
    }
  }
  pub fn is_full(&self) -> bool {
    if let Node::None = self.left {
      return false;
    } 
    if let Node::None = self.right {
      return false;
    }
    true
  }
  pub fn insert(&mut self, txn: Transaction) -> bool {
    // TODO: implement insert
    false
  }
}

#[derive(Debug)]
pub enum Node {
  Leaf(Box<LeafNode>),
  Internal(Box<InternalNode>),
  None,
}

impl Node {
  pub fn insert(&mut self, txn: Transaction) -> bool {
    match self {
      Node::Leaf(node) => node.insert(txn),
      Node::Internal(node) => node.insert(txn),
      Node::None => false // TODO: create internal node
    }
  }
  pub fn is_full(&self) -> bool {
    match self {
      Node::Leaf(_) => true,
      Node::Internal(node) => node.is_full(),
      Node::None => true
    }
  }
  pub fn traverse(&self, pre_cb: TraverseCb, in_cb: TraverseCb, post_cb: TraverseCb) {
    match self {
      Node::Leaf(leaf) => (),
      Node::Internal(node) => {
        pre_cb(self);
        Node::traverse(&node.left, pre_cb, in_cb, post_cb);
        in_cb(self);
        Node::traverse(&node.right, post_cb, in_cb, post_cb);
        ()
      },
      Node::None => ()
    }
  }
}


impl Hash for Node {
  fn hash<H: Hasher>(&self, s: &mut H) {
    match self {
      Node::Leaf(trx) => trx.hash(s),
      Node::Internal(node) => node.hash(s),
      Node::None => 0.hash(s)
    }
  }
}