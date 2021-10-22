use puzzles::read::readone;
use puzzles::read::readvec;

fn main() {
    let mut s = String::new();
    let a = readone::<i32>(&mut s);
    for _ in 0..a {
        let v = readvec::<i32>(&mut s);
        let b = v[0];
        let c = v[1];
        let d = v[2];
        if b == c-d {
            println!("does not matter");
        }
        else if b > c-d {
            println!("do not advertise");
        }
        else {
            println!("advertise")
        }
    }
}
