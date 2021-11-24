use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let times = readone::<i32>(&mut s);
    if times % 2 == 1 {
        println!("still running");
        return;
    }
    let total = (0..times / 2)
        .map(|_| (readone::<i32>(&mut s) - readone::<i32>(&mut s)).abs())
        .sum::<i32>();
    println!("{}", total);
}
