use std::error::Error;
use std::io::{Error as IoError, ErrorKind};

fn main() {
    let s: i32 = test().unwrap();
}

fn test() -> Result<i32, Box<dyn Error>> {
    // return Ok(456);
    Err(IoError::new(ErrorKind::Other, "some error").into())
}
