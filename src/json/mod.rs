use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum JSON<'a> {
  Number(i64),
  String(&'a str),
  Boolean(bool),
  Object(HashMap<&'a str, JSON<'a>>),
  Array(Vec<JSON<'a>>),
  Null
}

pub fn test_json () {
  let mut obj1 = HashMap::new();
  obj1.insert("key1", JSON::Number(32));
  obj1.insert("key2", JSON::Boolean(true));
  obj1.insert("key3", JSON::String("hello world"));
  obj1.insert("key4", JSON::Null);
  obj1.insert("key5", JSON::Array(vec![
    JSON::Boolean(false),
    JSON::String("hi foks")
  ]));

  let mut nested_obj1 = HashMap::new();
  nested_obj1.insert("nested1", JSON::Null);
  nested_obj1.insert("nested2", JSON::Array(vec![
    JSON::Null,
    JSON::String("some nested content")
  ]));

  obj1.insert("key6", JSON::Object(nested_obj1));

  let json = JSON::Object(obj1);
  println!("Our JSON object: {:#?}", json);
}
