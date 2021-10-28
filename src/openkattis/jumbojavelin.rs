use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let pieces = readone::<i32>(&mut s);
    let total = (0..pieces).map(|_| readone::<i32>(&mut s)).sum::<i32>();
    println!("{}", total - (pieces - 1));
}
