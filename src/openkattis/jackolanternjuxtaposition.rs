use puzzles::read::readmore;

fn main() {
    let mut s = String::new();
    println!("{}", readmore::<i32>(&mut s).product::<i32>());
}
