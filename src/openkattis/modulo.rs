use puzzles::read::readone;
use std::collections::HashSet;

fn main() {
    let mut s = String::new();
    let mut unique = HashSet::new();
    for _ in 0..10 {
        let modulo = readone::<i32>(&mut s) % 42;
        unique.insert(modulo);
    }
    println!("{}", unique.len());
}
