use crate::util::read::read;

pub fn solve(a: i32, b: i32) -> i32 {
    return a + b;
}

#[allow(dead_code)]
pub fn main() {
    let mut s = String::new();
    solve(read::<i32>(&mut s));
}
