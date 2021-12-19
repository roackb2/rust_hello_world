use std::collections::HashMap;

#[derive(Debug)]
enum JSON {
  Number(i64),
  String(String),
  Boolean(bool),
  Object(HashMap<String, JSON>),
  Array(Vec<JSON>),
  Null
}

pub fn test_json () {
  let mut obj1 = HashMap::new();
  obj1.insert(String::from("key1"), JSON::Number(32));
  obj1.insert(String::from("key2"), JSON::Boolean(true));
  obj1.insert(String::from("key3"), JSON::String(String::from("hello world")));
  obj1.insert(String::from("key4"), JSON::Null);
  obj1.insert(String::from("key5"), JSON::Array(vec![
    JSON::Boolean(false),
    JSON::String(String::from("hi foks"))
  ]));

  let mut nested_obj1 = HashMap::new();
  nested_obj1.insert(String::from("nested1"), JSON::Null);
  nested_obj1.insert(String::from("nested2"), JSON::Array(vec![
    JSON::Null,
    JSON::String(String::from("some nested content"))
  ]));

  obj1.insert(String::from("key6"), JSON::Object(nested_obj1));

  let json = JSON::Object(obj1);
  println!("Our JSON object: {:#?}", json);
}
