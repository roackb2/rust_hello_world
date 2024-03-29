use std::cmp::Ordering;
use super::link::Link;
use super::types::{ Value, TraverseCb, CollectCb };

#[derive(Debug)]
pub struct Node<T: Value> {
  key: u32,
  value: T,
  left: Link<T>,
  right: Link<T>
}

impl<T: Value> Node<T> {
  pub fn new(key: u32, value: T) -> Node<T> {
    Node {
      key,
      value,
      left: Link::None,
      right: Link::None
    }
  }
  pub fn key(&self) -> u32 { self.key }
  pub fn value(&self) -> T { self.value.clone() }
  pub fn update(&mut self, value: T) {
    self.value = value;
  }
  pub fn traverse(&self, pre: Option<TraverseCb<T>>, mid: Option<TraverseCb<T>>, post: Option<TraverseCb<T>>) {
    if let Some(cb) = pre { cb(self) };
    if let Link::To(left) = &self.left {
      Node::traverse(&left, pre, mid, post);
    }
    if let Some(cb) = mid { cb(self) };
    if let Link::To(right) = &self.right {
      Node::traverse(&right, pre, mid, post);
    }
    if let Some(cb) = post { cb(self) };
  }
  pub fn collect(
    &self,
    state: &mut Vec<T>,
    pre: Option<CollectCb<T>>,
    mid: Option<CollectCb<T>>,
    post: Option<CollectCb<T>>,
  ) {
    if let Some(cb) = pre {
      cb(self, state);
    }
    if let Link::To(left) = &self.left {
      left.collect(state, pre, mid, post);
    }
    if let Some(cb) = mid {
      cb(self, state);
    }
    if let Link::To(right) = &self.right {
      right.collect(state, pre, mid, post);
    }
    if let Some(cb) = post {
      cb(self, state);
    }
  }
  pub fn search(&self, key: u32) -> Option<T> {
    match key.cmp(&self.key) {
      Ordering::Equal => Some(self.value.clone()),
      Ordering::Greater => self.right.search(key),
      Ordering::Less => self.left.search(key)
    }
  }
  pub fn insert(&mut self, key: u32, value: T) -> bool {
    match key.cmp(&self.key) {
      Ordering::Equal => {
        self.update(value);
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