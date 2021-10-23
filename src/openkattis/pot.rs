use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let a = readone::<i32>(&mut s);
    let mut x = 0;
    for _ in 0..a {
        let b = readone::<u32>(&mut s);
        let c = b / 10;
        let d = b % 10;
        x += c.pow(d);
    }
    println!("{}", x);
}
