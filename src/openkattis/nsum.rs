use crate::util::read::readone;
use crate::util::read::readmore;
use std::iter::Iterator;

#[allow(dead_code)]
pub fn main() {
    let mut s = String::new();
    readone::<i32>(&mut s);
    let v = readmore::<i32>(&mut s).sum::<i32>();
    println!("{}",v)
}