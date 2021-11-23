use puzzles::read::readvec;

fn main() {
    let mut dice = readvec::<i32>(&mut String::new());
    dice.sort();
    for k in dice[0] + 1..=dice[1] + 1 {
        println!("{}", k);
    }
}
