use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let original = readone::<i32>(&mut s);
    let first = original / 10;
    let second = original % 10;
    println!("{}{}", second, first);
}
