#[cfg(test)]
pub mod tree_traversal {
  use std::collections::HashMap;
  use rust_hello_world::json::JSON;
  use rust_hello_world::bst::{
    b_tree::BTree,
    node::Node,
    types::CollectCb
  };

  const USERNAME: &str = "username";

  fn gen_user_obj<'a>(username: &'a str) -> JSON<'a> {
    let mut obj: HashMap<&'a str, JSON<'a>> = HashMap::new();
    obj.insert(USERNAME, JSON::String(username));
    JSON::Object(obj)
  }

  fn setup<'a>() -> (
    BTree<JSON<'a>>,
    Vec<JSON<'a>>,
    CollectCb<JSON<'a>>
  ) {
    let mut tree = BTree::new(10, gen_user_obj("god"));
    tree.insert(5, gen_user_obj("alice"));
    tree.insert(15, gen_user_obj("bob"));
  
    let state: Vec<JSON<'a>> = vec![];
  
    fn cb<'a>(node: &Node<JSON<'a>>, state: &mut Vec<JSON<'a>>) {
      state.push(node.value());
    }
  
    (tree, state, cb)
  }

  fn get_json_value<'a>(json: JSON<'a>) -> &str {
    match json {
      JSON::Object(obj) => match obj.get(USERNAME).unwrap() {
        JSON::String(s) => s,
        _ => panic!("expected &str to be content of JSON object")
      },
      _ => panic!("expect JSON object")
    }
  }

  fn extract_names<'a>(items: Vec<JSON<'a>>) -> Vec<&'a str> {
    items.into_iter()
      .map(|json| get_json_value(json))
      .collect()
  }

  #[test]
  fn pre_order() {
    let (tree, mut state, cb) = setup();
    tree.collect(&mut state, Some(cb), None, None);
    let names: Vec<&str> = extract_names(state);
    assert_eq!(names, vec!["god", "alice", "bob"]);
  }

  #[test]
  fn in_order() {
    let (tree, mut state, cb) = setup();
    tree.collect(&mut state, None, Some(cb), None);
    let names: Vec<&str> = extract_names(state);
    assert_eq!(names, vec!["alice", "god", "bob"]);
  }

  #[test]
  fn post_order() {
    let (tree, mut state, cb) = setup();
    tree.collect(&mut state, None, None, Some(cb));
    let names: Vec<&str> = extract_names(state);
    assert_eq!(names, vec!["alice", "bob", "god"]);
  }
}