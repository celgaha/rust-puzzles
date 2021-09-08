use crate::util::is_divisible;

#[allow(dead_code)]
pub fn solve(a: i32) -> i32 {
    let mut m = 0;
    for n in 1..a {
        if is_divisible(n, 3) || is_divisible(n, 5) {
            m += n;
        }
    }
    m
}

#[test]
fn test() {
    assert_eq!(solve(10), 23);
}
