use crate::util::is_divisible;

#[allow(dead_code)]
pub fn solve(n: i32) {
    if is_divisible(n, 2) {
        println!("Bob");
    } else {
        println!("Alice");
    }
}

#[test]
fn test_print() {
    assert_eq!(solve(5), println!("Alice"));
    assert_eq!(solve(8), println!("Bob"));
}
