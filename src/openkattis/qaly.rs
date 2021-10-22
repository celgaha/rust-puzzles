use puzzles::read::readmore;
use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let a = readone::<i32>(&mut s);
    let t = (0..a)
        .map(|_| readmore::<f64>(&mut s).product::<f64>())
        .sum::<f64>();
    println!("{}", t)
}
