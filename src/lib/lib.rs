pub mod read;

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

pub fn i32_to_binary(a: i32) -> Vec<bool> {
    let mut v = Vec::new();
    let mut b = a;
    while b > 0 {
        if is_divisible(b, 2) {
            v.push(false);
        } else {
            v.push(true);
        }
        b = b / 2;
    }
    v
}

pub fn binary_to_i32(v: &[bool]) -> i32 {
    let mut a = 0;
    for (k, b) in v.iter().enumerate() {
        if *b {
            a += 2i32.pow(k as u32);
        }
    }
    a
}

pub fn contains<T: std::cmp::PartialEq>(k: &T, v: &[T]) -> bool {
    for n in v {
        if *k == *n {
            return true;
        }
    }
    false
}

#[test]
fn test_i32_to_binary() {
    assert_eq!(i32_to_binary(6), vec![false, true, true]);
    assert_eq!(i32_to_binary(9), vec![true, false, false, true]);
    assert_eq!(i32_to_binary(0), vec![]);
    assert_eq!(i32_to_binary(1), vec![true]);
}
#[test]
fn test_binary_to_i32() {
    assert_eq!(binary_to_i32(&[false, true, true]), 6);
    assert_eq!(binary_to_i32(&[true, false, false, true]), 9);
    assert_eq!(binary_to_i32(&[false]), 0);
    assert_eq!(binary_to_i32(&[true]), 1);
    assert_eq!(binary_to_i32(&[]), 0);
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
