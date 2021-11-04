use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let cases = readone::<i32>(&mut s);
    for _ in 0..cases {
        let number = readone::<i32>(&mut s);
        let outcome = (1..=number).product::<i32>();
        println!("{}", outcome % 10);
    }
}
