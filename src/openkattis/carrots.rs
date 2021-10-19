use crate::util::read::readvec;

#[allow(dead_code)]
pub fn main() {
    let mut s = String::new();
    let v = readvec::<i32>(&mut s);
    println!("{}", v[1])
}
