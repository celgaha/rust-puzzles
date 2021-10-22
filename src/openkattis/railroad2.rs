use puzzles::read::readmore;

fn main() {
    let mut s = String::new();
    let v = readmore::<i32>(&mut s).nth(1).unwrap();
    if v % 2 == 0 {
        println!("possible");
    } else {
        println!("impossible");
    }
}
