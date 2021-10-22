use puzzles::read::readmore;
use puzzles::read::readone;

fn adventure(a: String) -> bool {
    todo!();
}

fn main() {
    let mut s = String::new();
    let a = readone::<i32>(&mut s);
    for _ in 0..a {
        if adventure(readone::String(&mut s)) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
