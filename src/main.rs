fn main() {
    let mut m = 0;
    for n in 1..1000{
        if is_divisible(n, 3) || is_divisible(n, 5) {
        println!("n={}",n);
        m += n;
        }
    }
    println!("Total = {}", m)
}

#[test]
fn test_divisible() {
    assert!(is_divisible(5, 5));
    assert!(!is_divisible(5, 3));
}

fn is_divisible(n: i32, k: i32) -> bool{
    n%k == 0
}