use puzzles::read::readmore;
use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    readone::<i32>(&mut s);
    let temps = readmore::<i32>(&mut s).filter(|&k| k < 0).count();
    println!("{}", temps)
}
