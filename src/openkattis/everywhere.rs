use puzzles::read::readone;
use std::collections::HashSet;

fn main() {
    let mut s = String::new();
    let cases = readone::<i32>(&mut s);
    for _ in 0..cases {
        let trips = readone::<i32>(&mut s);
        let mut cities = HashSet::new();
        for _ in 0..trips {
            cities.insert(readone::<String>(&mut s));
        }
        println!("{}", cities.len());
    }
}
