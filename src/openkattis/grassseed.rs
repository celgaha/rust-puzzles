use puzzles::read::{readmore, readone};

fn main() {
    let mut s = String::new();
    let cost = readone::<f64>(&mut s);
    let lawns = readone::<i32>(&mut s);
    let mut area: f64 = 0.0;
    for _ in 0..lawns {
        area += readmore::<f64>(&mut s).product::<f64>();
    }
    println!("{}", area * cost);
}
