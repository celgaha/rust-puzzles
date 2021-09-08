use crate::util::is_divisible;

pub fn solve() -> i32 {
    let mut m = 0;
    for n in 1..1000{
        if is_divisible(n, 3) || is_divisible(n, 5) {
            m += n;
        }
    }
    m
}
