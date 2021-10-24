use puzzles::read::readmore;

fn main() {
    let mut s = String::new();
    let mut maxpoints = 0;
    let mut winner = 0;
    for k in 0..5 {
        let points = readmore::<i32>(&mut s).sum::<i32>();
        if points > maxpoints {
            winner = k + 1;
            maxpoints = points;
        }
    }
    println!("{} {}", winner, maxpoints);
}
