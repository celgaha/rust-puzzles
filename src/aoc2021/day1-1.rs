use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let mut result = 0;
    let mut last = 0;
    for _ in 0..2000 {
        let depth = readone::<i32>(&mut s);
        if depth > last {
            result += 1;
        }
        last = depth;
    }
    println!("{}", result - 1);
}
