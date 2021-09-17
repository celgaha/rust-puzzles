pub fn solve(a: i32) -> i32 {
    let mut m = 0;
    let mut n = 0;
    let mut p = 0;
    for k in 1..a + 1 {
        m += k * k;
        n += k;
    }
    n = n * n;
    p = n - m;
    p
}

#[test]
fn test_square() {
    assert_eq!(solve(10), 2640);
}
