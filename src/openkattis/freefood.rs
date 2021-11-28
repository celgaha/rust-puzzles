use puzzles::read::{readone, readvec};
use std::collections::HashSet;

fn main() {
    let mut s = String::new();
    let events = readone::<i32>(&mut s);
    let mut result = HashSet::new();
    for _ in 0..events {
        let days = readvec::<i32>(&mut s);
        for k in days[0]..=days[1] {
            result.insert(k);
        }
    }
    println!("{}", result.len());
}
