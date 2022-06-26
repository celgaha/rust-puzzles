use puzzles::is_relatively_prime64;

#[allow(dead_code)]

pub fn solve(n: i64) -> i64 {
    let mut v: Vec<i64> = Vec::new();
    v.push(2);
    for k in (3..n).step_by(2) {
        if is_relatively_prime64(k, &v) {
            v.push(k);
            println!("{}", k);
        }
    }
    v.iter().sum()
}

#[test]
fn test() {
    assert_eq!(solve(10), 17);
}
