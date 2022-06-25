use puzzles::{is_relatively_prime, is_divisible64};

#[allow(dead_code)]
pub fn solve(mut a: i64) -> i64 {
    let mut v: Vec<i32> = Vec::new();   
        for k in 2..a {
            if is_relatively_prime(k.try_into().unwrap(), &v) {
                v.push(k.try_into().unwrap());
                while is_divisible64(a, k) {
                    a = a/k;
                }
            }
            println!("{}", a);
            if a <= 1 {
                break;
            }
        }
    let m = v.pop().unwrap();
    println!("{}", m);
    m.try_into().unwrap()
}

#[test]
fn test() {
    assert_eq!(solve(13195), 29);
}
