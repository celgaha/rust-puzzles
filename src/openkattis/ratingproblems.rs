use puzzles::read::{readone, readvec};

fn main() {
    let mut s = String::new();
    let v = readvec::<f64>(&mut s);
    let total = v[0];
    let judged = v[1];
    let left_over = total - judged;
    let mut current = 0.0;
    for _ in 0..judged as i32 {
        let rating = readone::<f64>(&mut s);
        current += rating;
    }
    let min: f64 = (current + left_over * -3.0) / total;
    let max: f64 = (current + left_over * 3.0) / total;
    println!("{} {}", min, max);
}
