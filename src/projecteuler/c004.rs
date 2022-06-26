#[allow(dead_code)]

pub fn solve(k: i32) -> i32 {
    let mut max = 0;
    for m in 1..k {
        for n in 1..m {
            let current = (m * n)
                .to_string()
                .chars()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            let reverse = current
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            if current == reverse && current > max {
                max = current;
            }
        }
    }
    max
}

#[test]
fn test() {
    assert_eq!(solve(100), 9009);
}
