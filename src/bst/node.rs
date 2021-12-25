use std::cmp::Ordering;
use super::link::Link;
use super::types::TraverseCb;

#[derive(Debug)]
pub struct Node<T: Copy> {
  key: u32,
  value: T,
  left: Link<T>,
  right: Link<T>
}

impl<T: Copy> Node<T> {
  pub fn new(key: u32, value: T) -> Node<T> {
    Node {
      key,
      value,
      left: Link::None,
      right: Link::None
    }
  }
  pub fn key(&self) -> u32 { self.key }
  pub fn value(&self) -> T { self.value }
  pub fn update(&mut self, value: T) {
    self.value = value;
  }
  pub fn traverse(&self, pre: TraverseCb<T>, mid: TraverseCb<T>, post: TraverseCb<T>) {
    pre(self);
    if let Link::To(left) = &self.left {
      Node::traverse(&left, pre, mid, post);
    }
    mid(self);
    if let Link::To(right) = &self.right {
      Node::traverse(&right, pre, mid, post);
    }
    post(self);
  }
  pub fn search(&self, key: u32) -> Option<T> {
    match key.cmp(&self.key) {
      Ordering::Equal => Some(self.value),
      Ordering::Greater => self.right.search(key),
      Ordering::Less => self.left.search(key)
    }
  }
  pub fn insert(&mut self, key: u32, value: T) -> bool {
    match key.cmp(&self.key) {
      Ordering::Equal => {
        self.value = value;
        true
      },
      Ordering::Greater => {
        match &mut self.right {
          Link::None => {
            self.right = Link::new(key, value);
            true
          }
          Link::To(node) => node.insert(key, value)
        }
      },
      Ordering::Less => {
        match &mut self.left {
          Link::None => {
            self.left = Link::new(key, value);
            true
          }
          Link::To(node) => node.insert(key, value)
        }
      }
    }
  }
}