use std::collections::hash_map::DefaultHasher;
use std::hash::{ Hash, Hasher };
use crate::structures::utils::{ get_hash };
use crate::structures::transaction::{ Transaction };
use crate::structures::node::{ Node, InternalNode, LeafNode };

#[derive(Debug)]
struct MerkleTree {
  root: Box<InternalNode>,
}

impl MerkleTree {
  pub fn new(data: Transaction) -> MerkleTree {
    let leaf = Node::Leaf(LeafNode::new(data));
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

pub fn test_tree() {
  let txn = Transaction::new(
    String::from("alice@good.com"),
    String::from("bob@good.com"),
    100
  );
  let mut tree = MerkleTree::new(txn);
  println!("testing merkle tree: {:#?}", tree);
  let transactions = [Transaction::new(
    String::from("bob@good.com"),
    String::from("john@great.com"),
    20
  ), Transaction::new(
    String::from("kate@awesome.com"),
    String::from("john@great.com"),
    35
  ), Transaction::new(
    String::from("somebody@awesome.com"),
    String::from("john@great.com"),
    35
  )];
  for txn in transactions {
    tree.insert(txn);
    println!("tree after insert transaction: {:#?}", tree);
  }
}