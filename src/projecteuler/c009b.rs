use std::collections::VecDeque;
use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Triple(i32, i32, i32);

impl Add for Triple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Triple(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Mul<i32> for Triple {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Triple(self.0 * other, self.1 * other, self.2 * other)
    }
}

#[allow(dead_code)]
pub fn solve(n: i32) -> Option<i32> {
    let mut to_check = VecDeque::new();
    to_check.push_back(Triple(3, 4, 5));

    let m0 = Triple(2, -2, -2);
    let m1 = Triple(1, 2, 1);
    let m2 = Triple(-1, 2, 3);

    while let Some(Triple(a, b, c)) = to_check.pop_front() {
        dbg!(a, b, c);
        if n % (a + b + c) == 0 {
            let m = n / (a + b + c);
            return Some(a*b*c*m*m*m)
        }
        to_check.push_back(m0 * a + m1 * b + m2 * c);
    }
    None
}

#[test]
fn test() {
    assert_eq!(solve(12), Some(60));
    assert_eq!(solve(1000), Some(31875000));
}
