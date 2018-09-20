use std::hash::{Hash,Hasher};
use std::collections::hash_map::DefaultHasher;

fn main() {
    let mut hash = DefaultHasher::new();
    "and".hash(&mut hash);
    println!("{}", hash.finish());
}
