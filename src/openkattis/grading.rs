use puzzles::read::{readone, readvec};

fn main() {
    let mut s = String::new();
    let percent = readvec::<i32>(&mut s);
    let exam = readone::<i32>(&mut s);
    if exam >= percent[0] {
        println!("A");
    } else if exam >= percent[1] {
        println!("B");
    } else if exam >= percent[2] {
        println!("C");
    } else if exam >= percent[3] {
        println!("D");
    } else if exam >= percent[4] {
        println!("E");
    } else {
        println!("F");
    }
}
