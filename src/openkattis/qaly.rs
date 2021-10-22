use puzzles::read::readone;
use puzzles::read::readvec;

fn main() {
    let mut s = String::new();
    let a = readone::<i32>(&mut s);
    let mut t = 0;
    for _ in 0..a {
        let v = readvec::<f64>(&mut s);
        let b = v[0];
        let c = v[1];
        t += b*c;
    }
    println!("{}", t)
}
