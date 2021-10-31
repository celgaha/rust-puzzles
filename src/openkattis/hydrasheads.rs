use puzzles::read::readvec;

fn steps(mut heads: i32, mut tails: i32) -> i32 {
    let mut tdiv = 2;
    let mut moves = 0;
    if heads % 2 == 0 {
        tdiv = 0;
    }
    while tails % 4 != tdiv {
        moves += 1;
        tails += 1;
    }
    moves += tails/2;
    heads += tails/2;
    moves += heads/2;
    moves
}

#[test]
fn test_steps() {
    assert_eq!(steps(3, 3), 9);
    assert_eq!(steps(1, 1), 3);
}

fn main() {
    let mut s = String::new();
    loop {
        let input = readvec::<i32>(&mut s);
        if input == [0, 0] {
            break;
        }
        let heads = input[0];
        let tails = input[1];
        println!("{}", steps(heads, tails));
    }
}
