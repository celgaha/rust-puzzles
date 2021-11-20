use puzzles::read::read_line;
use puzzles::read::readmore;
use std::collections::HashSet;

fn main() {
    let mut s = String::new();
    read_line(&mut s);
    let knots = readmore::<i32>(&mut s);
    let learned = readmore::<i32>(&mut String::new()).collect::<HashSet<_>>();
    for k in knots {
        if learned.contains(&k) == false {
            println!("{}", k);
        }
    }
}
