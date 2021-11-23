use puzzles::read::{readmore, readone};

fn main() {
    let mut s = String::new();
    let mut bats = readone::<f32>(&mut s);
    let runs = readmore::<f32>(&mut s);
    let mut sum = 0.0;
    for k in runs {
        if k == -1. {
            bats += -1.0;
            continue;
        }
        sum += k;
    }
    println!("{}", (sum / bats));
}
