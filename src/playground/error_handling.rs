use std::fs::{ self, File};
use std::io::{ self, ErrorKind, Read };

pub fn read_file_simple(path: &str) -> Result<String, io::Error> {
  fs::read_to_string(path)
}

pub fn read_file_error_chain(path: &str) -> Result<String, io::Error> {
  let mut s = String::new();
  File::open(path)?.read_to_string(&mut s)?;
  Ok(s)
}

pub fn test_errors() {
  let path = "./data/hello.txt";

  // A verbose implementation
  let mut f = File::open(&path).unwrap_or_else(|err| {
    if err.kind() == ErrorKind::NotFound {
      File::create(&path).unwrap_or_else(|err| {
        panic!("Fail to create file: {:?}", err)
      })
    } else {
      panic!("Failed to open file: {:?}", err)
    }
  });

  let mut content = String::new();
  match f.read_to_string(&mut content) {
    Ok(_) => (),
    Err(err) => panic!("Failed to read")
  }

  // Use a cleaner implementation in a function with error propagation
  match read_file_error_chain(&path) {
    Ok(content) => println!("We're good to read the file! content is: {}", content),
    Err(err) => println!("We got some trouble: {:?}", err)
  }

  // Or read the file using built-in function
  match fs::read_to_string(path) {
    Ok(content) => println!("Reading file using read_to_string, content is: {}", content),
    Err(err) => println!("We got some trouble: {:?}", err)
  }
}