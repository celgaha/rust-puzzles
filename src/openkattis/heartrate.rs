use puzzles::read::{readone, readvec};

fn main() {
    let mut s = String::new();
    let cases = readone::<i32>(&mut s);
    for _ in 0..cases {
        let measured = readvec::<f64>(&mut s);
        println!(
            "{} {} {}",
            60.0 / (measured[1] / (measured[0] - 1.0)),
            60.0 / (measured[1] / measured[0]),
            60.0 / (measured[1] / (measured[0] + 1.0))
        );
    }
}
