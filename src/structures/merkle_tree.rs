struct LeafNode {
  name: [char; 512],
  content_hash: [char; 128],
  meta_hash: [char; 128],
}

struct InternalNode {
  hash: [char; 128],
  left: Box<Node>,
  right: Box<Node>,
}

enum Node {
  Leaf(LeafNode),
  Internal(InternalNode),
}

struct MerkleTree {
  root: Box<Node>,
}

impl MerkleTree {
  pub fn new(node: LeafNode) -> MerkleTree {
    MerkleTree {
      root: Box::new(Node::Leaf(node))
    }
  }
}

pub fn test_tree() {
  println!("testing merkle tree");
}