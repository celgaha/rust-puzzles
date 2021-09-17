use crate::util::is_divisible;

pub fn solve(a: usize) -> i32 {
    let mut v: Vec<i32> = Vec::new();
    let mut p = 2;
    while v.len() < a {
        if !is_divisible(p, v) {
            v.push(p);
        }
        p += 1;
    }
    return v[a - 1];
}
