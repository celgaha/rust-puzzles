use puzzles::read::readmore;
use puzzles::read::readone;
use std::iter::Iterator;

pub fn main() {
    let mut s = String::new();
    readone::<i32>(&mut s);
    let v = readmore::<i32>(&mut s).sum::<i32>();
    println!("{}", v)
}
