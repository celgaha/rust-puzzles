use puzzles::{is_divisible64, is_prime};

#[allow(dead_code)]
pub fn solve(a: i64) -> i64 {
    let mut m = 0;

    for k in 1..a {
        if is_prime(k) {
            if is_divisible64(a, k) {
                m = k;
            }
        }
    }
    m
}

#[test]
fn test() {
    assert_eq!(solve(13195), 29);
}
