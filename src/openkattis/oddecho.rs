use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let length = readone::<i32>(&mut s);
    for _ in 0..length / 2 {
        println!("{}", readone::<String>(&mut s));
        readone::<String>(&mut s);
    }
    if length % 2 == 1 {
        println!("{}", readone::<String>(&mut s));
    }
}
