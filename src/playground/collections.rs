use std::collections::HashMap;

#[derive(Debug)]
struct NotifyPayload {
  event_name: String,
}

#[derive(Debug)]
struct StreamPayload {
  content: String
}

#[derive(Debug)]
enum Message {
  Notify(NotifyPayload),
  Stream(StreamPayload),
  Ack
}

pub fn test_collections() {
  let mut v: Vec<u32> = Vec::new();
  v.push(123);
  v.push(321);
  v.push(345);
  v.remove(1);
  println!("our number collection: {:#?}", v);

  let mut v = vec![5, 6, 7, 8, 9];
  v.push(10);
  v.splice(0..2, [1, 2]);
  println!("vector created using macro: {:#?}", v);

  let third: &i32 = &v[3];
  println!("the third element using array index: {}", third);

  match v.get(3) {
    Some(x) => println!("the third element using vector get: {}", x),
    None => println!("No element here!")
  }

  v.push(11);
  // NOTE: This won't work cuz the 'third' variable is a immutable borrow of array element
  // println!("the third element after array mutate: {}", third);

  for i in &mut v {
    *i += 10;
  }
  println!("vector altered by for loop: {:#?}", v);

  let mut v = vec![
    Message::Notify(NotifyPayload {
      event_name: "connect".to_string()
    }),
    Message::Stream(StreamPayload {
      content: "some media content".to_string()
    }),
    Message::Ack
  ];
  println!("a vector with multiple types: {:#?}", v);
}

pub fn test_strings() {
  let s1 = String::from("abc");
  let s2 = "def".to_string();
  let s = s1 + &s2;
  println!("string after add: {}", s);

  let s3 = "yo";
  let s4 = "hey";
  let s = format!("{}-{}-{}", s, s3, s4);
  println!("string concatenation using format: {},", s);

  // Print characters in string one by one.
  // NOTE: The 4th and 6th ones are diacritics,
  // which might be printed correctly on console. 
  println!("Characters of 'नमस्ते'");
  for c in "नमस्ते".chars() {
    println!("{}", c);
  }

  // Print raw bytes one by one in a String.
  println!("Bytes of 'नमस्ते'");
  for b in "नमस्ते".bytes()   {
    println!("{}", b);
  }
}

pub fn test_hashmap() {
  let mut m: HashMap<String, u32> = HashMap::new();
  m.insert("team 1".to_string(), 32);
  m.insert("team 2".to_string(), 28);
  println!("Scores of all teams: {:#?}", m);

  let teams = vec!["team 1", "team 2", "team 3"];
  let scores = [28, 32, 57];

  let mut m: HashMap<_, _> = 
    teams.into_iter().zip(scores.into_iter()).collect();
  println!("Scores using zip: {:#?}", m);

  // Update the score of team 2;
  m.insert("team 2", 81);
  println!("Scores after updating team 2: {:#?}", m);

  m.entry("team 4").or_insert(11);
  println!("Scores after entry insert: {:#?}", m);

  let corpus = "Hello world beautiful world wonderful Hello !";
  let mut tf_idf: HashMap<&str, u32> = HashMap::new();
  for word in corpus.split_whitespace() {
    let count = tf_idf.entry(word).or_insert(0);
    *count += 1;
  }

  println!("tf-idf of our corpus: {:#?}", tf_idf);
}