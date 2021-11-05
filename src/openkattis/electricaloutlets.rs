use puzzles::read::{readone, readvec};

fn main() {
    let mut s = String::new();
    let times = readone::<i32>(&mut s);
    for _ in 0..times {
        let mut outlets = readvec::<i32>(&mut s);
        let number = outlets.remove(0);
        println!("{}", outlets.iter().sum::<i32>() - (number - 1));
    }
}
