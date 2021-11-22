use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let x = readone::<i32>(&mut s);
    let y = readone::<i32>(&mut s);
    let mut quadrant = 1;
    if x * y < 0 {
        quadrant += 1;
    }
    if y < 0 {
        quadrant += 2;
    }
    println!("{}", quadrant);
}
