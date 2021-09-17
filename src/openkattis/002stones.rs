use crate::util::is_divisible;

pub fn solve(N: i32) -> str {
    if is_divisible(N, 2) {
        return Bob;
    }
    return Alice;
}