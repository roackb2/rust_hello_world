extern crate num;

use num::traits::Float;

#[derive(Debug)]
struct Point<T> {
  x: T,
  y: T
}

impl<T: PartialOrd + Copy> Point<T> {
  pub fn clone(&self) -> Point<T> {
    Point { x: self.x, y: self.y }
  }
  pub fn upper_right(list: &[Point<T>]) -> Point<T> {
    let mut outest = list[0].clone();

    for item in list {
      if item.x > outest.x && item.y > outest.y {
        outest = item.clone();
      }
    }
    outest
  }
}

impl<T: Float> Point<T> {
  pub fn new(x: T, y: T) -> Point<T> {
    Point { x, y}
  }
  pub fn dist(&self, other: &Point<T>) -> T {
    ((self.x - other.x).exp2() + (self.y - other.y).exp2()).sqrt()
  }
}

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];
  
  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

pub trait EasyPrime {
  fn is_easy_prime(&self) -> bool;
}

impl EasyPrime for i32 {
  fn is_easy_prime(&self) -> bool {
    for num in vec![2, 3, 5, 7, 11, 13 ,17 ,19] {
      if *self % num == 0 {
        return true
      }
    }
    false
  }
}

impl Point<i32> {
  pub fn new_int(x: i32, y: i32) -> Point<i32> {
    Point { x, y }
  }
}

impl EasyPrime for Point<i32> {
  fn is_easy_prime(&self) -> bool {
    self.x.is_easy_prime() && self.y.is_easy_prime()
  }
}

pub fn test_generics() {
  let num_list = vec![9, 7, 6, 4, 3, 1, 5];

  let largest_num = largest(&num_list);
  println!("largest number in list: {}", largest_num);

  let char_list = vec!['a', 'e', 'z', 'g', 'e', 'f'];

  let largest_char = largest(&char_list);
  println!("largest character in list: {}", largest_char);

  let p1 = Point::new(23.5, 31.6);
  let p2 = Point::new(36.8, 64.2);
  let dist = p1.dist(&p2);
  println!("distance between p1 and p2: {}", dist);

  let points = vec![
    Point::new(2.0, 5.0),
    Point::new(3.0, 7.0),
    Point::new(8.0, 2.0),
    Point::new(3.0, 9.0),
    Point::new(11.0, 12.0)
  ];

  let most_upper_right = Point::upper_right(&points);
  println!("The most upper right point in list: {:#?}", most_upper_right);

  for num in vec![247, 249, 251, 257] {
    println!("is {} easy prime: {}", num, num.is_easy_prime());
  }

  let p = Point::new_int(251, 257);
  println!("is p a point {:#?} consists of easy prime: {}", p, p.is_easy_prime());
}