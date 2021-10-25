use puzzles::read::readmore;

fn main() {
    let mut s = String::new();
    let area: f64 = readmore::<f64>(&mut s).product();
    println!("{}", area / 2.0);
}
