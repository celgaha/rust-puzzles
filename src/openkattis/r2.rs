use puzzles::read::readvec;

fn main() {
    let mut s = String::new();
    let input = readvec::<i32>(&mut s);
    println!("{}", (input[1] - input[0]) * 2 + input[0]);
}
