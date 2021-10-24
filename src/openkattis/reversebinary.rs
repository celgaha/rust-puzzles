use puzzles::binary_to_i32;
use puzzles::i32_to_binary;
use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let a = readone::<i32>(&mut s);
    let b = i32_to_binary(a);
    let c = b.iter().rev().copied().collect::<Vec<bool>>();
    let d = binary_to_i32(&c);
    println!("{}", d)
}
