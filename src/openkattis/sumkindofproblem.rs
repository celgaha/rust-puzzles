use puzzles::read::{readone, readvec};

fn main() {
    let mut s = String::new();
    let times = readone::<i32>(&mut s);
    for _ in 0..times {
        let mut dataset = readvec::<i32>(&mut s);
        let int = dataset.pop().unwrap();
        let mut possum = (int * int) / 2 + int / 2;
        if int%2 == 1 {
            possum += 1;
        }
        let oddsum = int * int;
        let evensum = oddsum + int;
        println!("{} {} {} {}", dataset[0], possum, oddsum, evensum);
    }
}
