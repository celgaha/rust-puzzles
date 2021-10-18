use crate::util::is_divisible;
use crate::util::read::readone;

#[allow(dead_code)]
pub fn solve(n: i32) {
    if is_divisible(n, 2) {
        println!("Bob");
    } else {
        println!("Alice");
    }
}

#[allow(dead_code)]
pub fn main() {
    let mut s = String::new();
    solve(readone::<i32>(&mut s));
}

#[test]
fn test_print() {
    assert_eq!(solve(5), println!("Alice"));
    assert_eq!(solve(8), println!("Bob"));
}
