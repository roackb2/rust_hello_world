pub fn test_statement() {
  let x = {
      let y = 5;
      y + 1 // note that no semicolon after an expression
  };
  println!("Value of x: {}", x);
}

pub fn simple_add(a: i32, b: i32) -> i32 {
  println!("We're adding {} and {} with our simple_add function.", a, b);
  a + b
}

pub fn control_flow() {
  let cond = 5;
  // NOTE: No parenthesis needed for conditions
  let res = if cond < 8 {
      println!("Number is less than 8");
      true
  } else {
      println!("Number is greater than 8");
      false
  };
  println!("Assignment of the result for if-else expression: {}", res);
  let res = if cond < 8 { "Number is less than 8" } else { "Number is greater than 8" };
  println!("Assignment of single line if-else expression: {}", res);

  let mut x = 0;
  // NOTE: Rust support loop labeling
  let (x, y) = 'outer_loop: loop {
      x = x + 1;
      println!("value of x: {}", x);
      let mut y = 0;
      loop {
          y = y + 1;
          println!("value of y: {}", y);
          if x > 30 {
              println!("break outer loop if x > 30");
              break 'outer_loop (x, y);
          }
          if y > 30 {
              println!("break inner loop if y > 30");
              break;
          }
      }
  };
  println!("Value of x y after loop: {}, {}", x, y);

  let mut i = 0;
  while i < 30 {
      i = i + 1;
  }
  println!("result of i after while loop: {}", i);

  let arr = [3, 5, 8, 13, 21];
  println!("Iterate through array elements.");
  for e in arr {
      println!("element: {}", e);
  }

  for count in (1..10).rev() {
      println!("{}", count);
  }
  println!("Liftoff!!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
    assert_eq!(simple_add(3, 5), 8);
  }

  #[test] #[ignore]
  fn should_fail() {
      panic!("This should fail!");
  }
}