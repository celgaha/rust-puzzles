use puzzles::read::readvec;

fn main() {
    let input = readvec::<i32>(&mut String::new());
    let first = (1..=input[2]).map(|k| k % input[0] == 0);
    let second = (1..=input[2]).map(|k| k % input[1] == 0);
    if first.zip(second).any(|(a, b)| a && b) {
        println!("yes");
    } else {
        println!("no");
    }
}
