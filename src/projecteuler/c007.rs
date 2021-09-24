use crate::util::is_relatively_prime;

pub fn solve(a: usize) -> i32 {
    let mut v: Vec<i32> = Vec::new();
    let mut p = 2;
    while v.len() < a {
        if is_relatively_prime(p, v) {
            v.push(p);
        }
        p += 1;
    }
    return v[a - 1];
}
