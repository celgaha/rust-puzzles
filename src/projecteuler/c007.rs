use puzzles::is_relatively_prime;

#[allow(dead_code)]
pub fn solve(a: usize) -> i32 {
    let mut v: Vec<i32> = Vec::new();
    let mut p = 2;
    while v.len() < a {
        if is_relatively_prime(p, &v) {
            v.push(p);
        }
        p += 1;
    }
    v[a - 1]
}
