pub fn is_divisible(n: i64, k: i64) -> bool {
    n % k == 0
}

#[test]
fn test_divisible() {
    assert!(is_divisible(5, 5));
    assert!(!is_divisible(5, 3));
}
