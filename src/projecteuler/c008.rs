use std::collections::VecDeque;

#[allow(dead_code)]

pub fn solve(s: &str) -> i64 {
    let mut total: i64 = 1;
    let v: Vec<char> = s.chars().collect::<Vec<_>>();
    let mut number = VecDeque::new();

    for k in 0..13 {
        number.push_back(v[k].to_digit(10).unwrap() as i64);
        total = number.iter().product::<i64>();
        println!("{}", total);
    }

    let mut max = total;

    for k in 14..1000 {
        number.pop_front();
        number.push_back(v[k].to_digit(10).unwrap() as i64);
        total = number.iter().product::<i64>();
        if total > max {
            max = total;
        }
        println!("{}", max);
    }

    max
}
