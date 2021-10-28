use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    for k in 0..readone::<i32>(&mut s) {
        println!("{} Abracadabra", k + 1);
    }
}
