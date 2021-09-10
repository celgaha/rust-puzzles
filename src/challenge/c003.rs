use crate::util::is_divisible;

#[allow(dead_code)]
pub fn solve(a: i64) -> i64 {
    let mut m = 0;
    for k in 1..(a - 1) {
        for p in 2..(k - 1) {
            is_divisible(k, p);
        }
        if is_divisible(a, k) {
            m = k;
        }
    }
    m
}

#[test]
fn test() {
    assert_eq!(solve(13195), 29);
}
