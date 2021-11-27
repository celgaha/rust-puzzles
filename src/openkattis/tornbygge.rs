use puzzles::read::{readone, readvec};

fn main() {
    let mut s = String::new();
    readone::<i32>(&mut s);
    let blocks = readvec::<i32>(&mut s);
    let towers = 1 + blocks
        .iter()
        .zip(blocks.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count();
    println!("{}", towers);
}
