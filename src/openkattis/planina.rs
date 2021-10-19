use crate::util::read::readone;

#[allow(dead_code)]
pub fn main() {
    let mut s = String::new();
    let a = readone::<i32>(&mut s);
    let b = (2 ^ a + 1) ^ 2;
    println!("{}", b)
}
