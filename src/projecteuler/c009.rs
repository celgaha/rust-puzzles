use std::collections::VecDeque;

#[allow(dead_code)]
pub fn solve(n: u32) -> u32 {
    let mut to_check = VecDeque::new();
    to_check.push_back(3);
    to_check.push_back(4);
    to_check.push_back(5);

    let mut a: u32 = 0;
    let mut b = 0;
    let mut c = 0;

    let mut newa = 0;
    let mut newb: u32 = 0;
    let mut newc = 0;

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
    let m = a * b * c;
    m
}

#[test]
fn test() {
    assert_eq!(solve(12), 60);
}
