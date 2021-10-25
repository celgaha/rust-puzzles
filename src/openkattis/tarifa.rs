use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let mbs = readone::<i32>(&mut s);
    let months = readone::<i32>(&mut s);
    let max = (months + 1) * mbs;
    let mut used = 0;
    for _ in 0..months {
        used += readone::<i32>(&mut s);
    }
    println!("{}", max - used);
}
