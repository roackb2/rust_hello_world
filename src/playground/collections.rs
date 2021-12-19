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
      event_name: String::from("connect")
    }),
    Message::Stream(StreamPayload {
      content: String::from("some media content")
    }),
    Message::Ack
  ];
  println!("a vector with multiple types: {:#?}", v);
}