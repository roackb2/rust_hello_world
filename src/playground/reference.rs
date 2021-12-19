pub fn reference() {
  fn takes_reference(s: &String) -> usize {
      s.len()
  }

  let txt = String::from("hello");
  let len = takes_reference(&txt);
  println!("length of {}: {}", txt, len);

  let mut txt = String::from("hello");

  // following code won't work
  // fn change_wont_work(s: &String) {
  //     s.push_str(", world");
  // }
  // change_wont_work(&txt);
  // println!("value after change: {}", txt);

  fn change_works(s: &mut String) {
      s.push_str(", world");
  }

  change_works(&mut txt);
  println!("value after change: {}", txt);

  {
      // NOTE: This works fine because ownership of pt1
      // ends after block scope
      let pt1 = &mut txt;
      println!("value of pt2: {}", pt1);
  }

  // NOTE: This cases [E0499] cannot borrow `txt` as mutable more than once at a time. 
  // let pt1 = &mut txt;
  // let pt2 = &mut txt;
  // println!("value of pt1 & pt2: {} {}", pt1, pt2);

  // NOTE: This cause [E0502] cannot borrow `txt` as mutable because it is also borrowed as immutable. 
  // let pt1 = &txt;
  // let pt2 = &mut txt;
  // println!("value of pt1 & pt2: {} {}", pt1, pt2);

  let pt1 = &txt;
  let pt2 = &txt;
  println!("value of pt1 & pt2: {}, {}", pt1, pt2);
  let pt3 = &mut txt;
  // NOTE: This works fine because lifetime of pt1 & pt2 already ends here
  println!("value of pt3: {}", pt3);

  // NOTE: This won't work cuz scope of s ends after function call,
  // so returning a reference of dropped variable is invalid.
  // fn danble() -> &String {
  //     let s = String::from("hello");
  //     &s
  // }

  // NOTE: This works fine because we're returning the ownership
  fn no_dangle() -> String {
      let s = String::from("hello");
      s
  }
  let txt = no_dangle();
  println!("value of txt created inside function: {}", txt);
}