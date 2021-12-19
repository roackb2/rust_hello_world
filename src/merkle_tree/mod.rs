pub mod tree;
pub mod transaction;
mod node;
mod utils;

use self::{
  transaction::Transaction,
  tree::MerkleTree
};

pub fn test_tree() {
  let txn = Transaction::new(
    "alice@good.com".to_string(),
    "bob@good.com".to_string(),
    100
  );
  let mut tree = MerkleTree::new(txn);
  println!("testing merkle tree: {:#?}", tree);
  let transactions = [Transaction::new(
    "bob@good.com".to_string(),
    "john@great.com".to_string(),
    20
  ), Transaction::new(
    "kate@awesome.com".to_string(),
    "john@great.com".to_string(),
    35
  ), Transaction::new(
    "somebody@awesome.com".to_string(),
    "john@great.com".to_string(),
    35
  )];
  for txn in transactions {
    tree.insert(txn);
    println!("tree after insert transaction: {:#?}", tree);
  }
}