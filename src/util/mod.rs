pub fn is_divisible(n: i32, k: i32) -> bool {
    n % k == 0
}

pub fn is_divisible64(n: i64, k: i64) -> bool {
    n % k == 0
}

#[allow(dead_code)]
pub fn is_prime(n: i64) -> bool {
    if n == 1 {
        return false;
    }
    for k in 2..n {
        if is_divisible64(n, k) {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
pub fn is_relatively_prime(n: i32, v: &[i32]) -> bool {
    for k in v {
        if is_divisible(n, *k) {
            return false;
        }
    }
    true
}

#[test]
fn test_is_kinda_prime() {
    assert!(!is_relatively_prime(9, &[2, 3, 5, 7]));
    assert!(is_relatively_prime(7, &[2, 3, 5]));
    assert!(is_relatively_prime(2, &[]));
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
