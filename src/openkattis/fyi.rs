use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    if readone::<i32>(&mut s) / 10000 == 555 {
        println!("{}", 1);
    } else {
        println!("{}", 0);
    }
}
