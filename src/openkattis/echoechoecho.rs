#[allow(dead_code)]
fn iu() {
    println!(".iusai la celga'a cu zabna")
}

use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let e = readone::<String>(&mut s);
    println!("{} {} {}", e, e, e);
}
