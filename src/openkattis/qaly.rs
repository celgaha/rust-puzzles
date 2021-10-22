use puzzles::util::read::readone;
use puzzles::util::read::readvec;

fn main() {
    let mut s = String::new();
    let a = readone::<i32>(&mut s);
    let mut t = 0;
    for _ in 1..a {
        let v = readvec::<i32>(&mut s);
        let b = v[0];
        let c = v[1];
        t += b*c;
    }
    println!("{}", t)
}
