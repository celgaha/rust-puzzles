use crate::util::read::readone;

#[allow(dead_code)]
pub fn main() {
    let mut s = String::new();
    let a = readone::<u32>(&mut s);
    let b = (2i32.pow(a) + 1).pow(2);
    println!("{}", b)
}
