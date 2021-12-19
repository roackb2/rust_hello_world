pub fn ownership() {
  let s1 = String::from("hello");
  let s2 = s1; // ownership moved from s1 to s2
  // This would cause error: [E0382] borrow of moved value: `s1`.
  // println!("value of s1: {}", s1);
  println!("value of s2: {}", s2);
  let s3 = s2.clone();
  println!("Value of s2: {}, and cloned version s3: {}", s2, s3);
  let x = 3;
  let y = x;
  println!("Primitive values are copied to stack so both x {} and y {} are valid", x, y);

  fn is_copy(i: i32) {
      println!("value of i: {}", i);
  }

  fn takes_ownership(s: String) {
      println!("value of s: {}", s);
  }

  fn takes_and_giveback(s: String) -> String {
      println!("value of s: {}", s);
      s
  }

  let txt = String::from("hello");
  let i1 = 5;

  takes_ownership(txt);
  is_copy(i1);

  println!("value of i1 after function call: {}", i1);
  // NOTE: This cause runtime error 'borrow of moved value'
  // println!("value of txt after function call: {}", txt);

  let txt = String::from("hello");
  let txt = takes_and_giveback(txt);
  println!("value of txt after takes and give back ownership: {}", txt);


}