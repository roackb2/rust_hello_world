use super::link::Link;
use super::node::Node;
use super::types::{ Value, TraverseCb, CollectCb };

#[derive(Debug)]
pub struct BTree<T: Value> {
  root: Link<T>
}

impl<T: Value> BTree<T> {
  pub fn new(key: u32, value: T) -> BTree<T> {
    BTree {
      root: Link::new(key, value)
    }
  }
  pub fn search(&self, key: u32) -> Option<T> { self.root.search(key) }
  pub fn insert(&mut self, key: u32, value: T) -> bool {
    self.root.insert(key, value)
  }
  pub fn traverse(&self, pre: Option<TraverseCb<T>>, mid: Option<TraverseCb<T>>, post: Option<TraverseCb<T>>) {
    self.root.traverse(pre, mid, post);
  }
  pub fn collect(
    &self,
    state: &mut Vec<T>,
    pre: Option<CollectCb<T>>,
    mid: Option<CollectCb<T>>,
    post: Option<CollectCb<T>>,
  ) {
    self.root.collect(state, pre, mid, post);
  }
}

#[cfg(test)]
pub mod tree_traversal {
  use super::{
    BTree,
    Node,
    CollectCb
  };

  fn setup<'a>() -> (
    BTree<&'a str>,
    Vec<&'a str>,
    CollectCb<&'a str>
  ) {
    let mut tree = BTree::new(10, "root");
    tree.insert(5, "left");
    tree.insert(15, "right");
  
    let state: Vec<&str> = vec![];
  
    fn cb<'a>(node: &Node<&'a str>, state: &mut Vec<&'a str>) {
      state.push(node.value());
    }
  
    (tree, state, cb)
  }

  #[test]
  fn pre_order() {
    let (tree, mut state, cb) = setup();
    tree.collect(&mut state, Some(cb), None, None);
    assert_eq!(state, vec!["root", "left", "right"]);
  }

  #[test]
  fn in_order() {
    let (tree, mut state, cb) = setup();
    tree.collect(&mut state, None, Some(cb), None);
    assert_eq!(state, vec!["left", "root", "right"]);
  }

  #[test]
  fn post_order() {
    let (tree, mut state, cb) = setup();
    tree.collect(&mut state, None, None, Some(cb));
    assert_eq!(state, vec!["left", "right", "root"]);
  }
}