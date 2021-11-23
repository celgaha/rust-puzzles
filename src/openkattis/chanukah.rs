use puzzles::read::{readone, readvec};

fn main() {
    let mut s = String::new();
    for _ in 0..readone::<i32>(&mut s) {
        let v = readvec::<i32>(&mut s);
        let candles = (v[1] * v[1] + v[1]) / 2 + v[1];
        println!("{} {}", v[0], candles);
    }
}
