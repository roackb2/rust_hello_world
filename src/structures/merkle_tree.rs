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
}

pub fn test_tree() {
  let txn = Transaction::new(
    String::from("alice@good.com"),
    String::from("bob@good.com"),
    100
  );
  let tree = MerkleTree::new(txn);
  println!("testing merkle tree: {:#?}", tree);
}