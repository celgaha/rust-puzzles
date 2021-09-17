use crate::util::is_divisible;

#[allow(dead_code)]
pub fn solve(a: i64) -> i64{
    let mut m = a;
    for k in 1..a { 
        if !is_divisible(m,a-k) {
            m += a;
        }
    }
    m
}

#[test]
fn test_divisor() {
    assert_eq!(solve(10),2520);
}