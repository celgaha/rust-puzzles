pub fn is_divisible(n: i64, k: i64) -> bool {
    n % k == 0
}

#[allow(dead_code)]
pub fn is_prime(n: i64) -> bool {
    if n == 1 {
        return false;
    }
    for k in 2..n {
        if is_divisible(n, k) {
            return false;
        }
    }
    true
}

#[test]
fn test_divisible() {
    assert!(is_divisible(5, 5));
    assert!(!is_divisible(5, 3));
}

#[test]
fn test_prime() {
    assert!(!is_prime(9));
    assert!(is_prime(7));
    assert!(!is_prime(1));
}
