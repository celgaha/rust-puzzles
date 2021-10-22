use puzzles::util::is_divisible64;

#[allow(dead_code)]
pub fn solve(a: i64) -> i64 {
    let mut m = 0;
    for n in 1..a {
        if is_divisible64(n, 3) || is_divisible64(n, 5) {
            m += n;
        }
    }
    m
}

#[test]
fn test() {
    assert_eq!(solve(10), 23);
}
