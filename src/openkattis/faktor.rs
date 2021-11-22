use puzzles::read::readvec;

fn main() {
    let mut s = String::new();
    let input = readvec::<i32>(&mut s);
    println!("{}", input[0] * (input[1] - 1) + 1);
}
