use std::collections::VecDeque;

#[allow(dead_code)]
pub fn solve(n: i32) -> i32 {
    let mut to_check = VecDeque::new();
    to_check.push_back(3);
    to_check.push_back(4);
    to_check.push_back(5);

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    let mut newa;
    let mut newb;
    let mut newc;

    while a + b + c != n {
        a = to_check.pop_front().unwrap();
        b = to_check.pop_front().unwrap();
        c = to_check.pop_front().unwrap();

        newa = 2 * a + b - c;
        newb = 2 * b - 2 * a + 2 * c;
        newc = b - 2 * a + 3 * c;
        to_check.push_back(newa);
        to_check.push_back(newb);
        to_check.push_back(newc);

        newa = 2 * a + b + c;
        newb = 2 * a - 2 * b + 2 * c;
        newc = 2 * a - b + 3 * c;
        to_check.push_back(newa);
        to_check.push_back(newb);
        to_check.push_back(newc);

        newa = 2 * a - b + c;
        newb = 2 * a + 2 * b + 2 * c;
        newc = 2 * a + b + 3 * c;
        to_check.push_back(newa);
        to_check.push_back(newb);
        to_check.push_back(newc);
    }
    let m = 25 * a * 25 * b * 25 * c;
    m
}

#[test]
fn test() {
    assert_eq!(solve(12), 60);
}
