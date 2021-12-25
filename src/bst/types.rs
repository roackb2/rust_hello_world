use super::node::Node;

pub type TraverseCb<T> = fn(&Node<T>);