use puzzles::contains;
use puzzles::read::readone;
use puzzles::read::readvec;

fn main() {
    let mut s = String::new();
    let v = readvec::<usize>(&mut s);
    let num_parts = v[0];
    let days = v[1];
    let mut last_day = days;
    let mut log = Vec::new();
    for k in 0..days {
        let part = readone::<String>(&mut s);
        if contains(&part, &log) {
            continue;
        }
        log.push(part);
        last_day = k + 1;
    }
    if log.len() < num_parts {
        println!("paradox avoided");
    } else {
        println!("{}", last_day);
    }
}
