pub mod tree;
pub mod transaction;
pub mod node;
pub mod utils;

use crate::merkle_tree::transaction::Transaction;
use crate::merkle_tree::tree::MerkleTree;

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