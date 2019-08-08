use fluence::sdk::*;
use std::fs::File;
use std::io::prelude::*;

#[invocation_handler]
fn main() -> String {
    let mut file = File::create("foo.txt").unwrap();
    file.write_all(b"Hello, world!").unwrap();

    let mut file = File::open("foo.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    assert_eq!(contents, "Hello, world!");

    "success!".to_string()
}
