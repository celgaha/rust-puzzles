use crate::util::read::readmore;

#[allow(dead_code)]
pub fn main() {
    let mut s = String::new();
    let v = readmore::<i32>(&mut s);
    println!("{}", v.sum::<i32>());
}
