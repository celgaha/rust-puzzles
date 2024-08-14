use puzzles::read::{readtriple, readvec};

fn main() {
    let (total, broken, reserve) = readtriple::<i32>(&mut String::new());
    let mut bplaces = readvec::<i32>(&mut String::new());
    let mut rplaces = readvec::<i32>(&mut String::new());
}