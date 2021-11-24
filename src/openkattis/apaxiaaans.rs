use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let word = readone::<String>(&mut s);
    let mut short = String::new();
    for (a, b) in word.chars().zip(word.chars().skip(1)) {
        if a != b {
            short.push(a);
        }
    }
    short.push(word.chars().rev().nth(0).unwrap());
    println!("{}", short);
}
