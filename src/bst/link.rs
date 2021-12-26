use super::node::Node;
use super::types::{ TraverseCb, CollectCb };

#[derive(Debug)]
pub enum Link<T: Copy> {
  None,
  To(Box<Node<T>>)
}

impl<T: Copy> Link<T> {
  pub fn new(key: u32, value: T) -> Link<T> {
    Link::To(Box::new(Node::new(key, value)))
  }
  pub fn search(&self, key: u32) -> Option<T> {
    match self {
      Link::To(node) => node.search(key),
      Link::None => None
    }
  }
  pub fn insert(&mut self, key: u32, value: T) -> bool {
    match self {
      Link::To(node) => node.insert(key, value),
      Link::None => panic!("Cannot insert a link of None")
    }
  }
  pub fn traverse(&self, pre: Option<TraverseCb<T>>, mid: Option<TraverseCb<T>>, post: Option<TraverseCb<T>>) {
    match self {
      Link::To(node) => node.traverse(pre, mid, post),
      Link::None => ()
    }
  }
  pub fn collect(
    &self,
    state: &mut Vec<T>,
    pre: Option<CollectCb<T>>,
    mid: Option<CollectCb<T>>,
    post: Option<CollectCb<T>>,
  ) {
    match self {
      Link::To(node) => node.collect(state, pre, mid, post),
      Link::None => ()
    }
  }
}
