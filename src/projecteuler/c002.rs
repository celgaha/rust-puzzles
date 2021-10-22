use puzzles::util::is_divisible64;

#[allow(dead_code)]
pub fn solve(k: i64) -> i64 {
    let mut a = 1;
    let mut b = 2;
    let mut m = 2;
    while (a + b) <= k {
        let n = a + b;
        a = b;
        b = n;
        if is_divisible64(n, 2) {
            m += n;
        }
    }
    m
}

#[test]
fn test() {
    assert_eq!(solve(100), 44);
}
