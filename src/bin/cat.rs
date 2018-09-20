use std::env;
use std::fs;

fn main() {
    let mut s = String::new();
    for argument in env::args().skip(1) {
        s.push_str(&fs::read_to_string(&argument).expect(&format!("Unable to read file: {}", argument)));
    }
}
