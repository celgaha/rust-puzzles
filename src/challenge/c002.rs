use crate::util::is_divisible;

#[allow(dead_code)]
pub fn solve(k: i32) -> i32 {
    let mut a = 1;
    let mut b = 2;
    let mut m = 2;
    while (a + b) <= k {
        let n = a + b;
        a = b;
        b = n;
        if is_divisible(n, 2) {
            m += n;
        }
    }
    m
}

#[test]
fn test() {
    assert_eq!(solve(100), 44);
}
