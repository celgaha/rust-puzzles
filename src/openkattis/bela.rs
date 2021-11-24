use puzzles::read::{readone, readvec};

fn main() {
    let mut s = String::new();
    let v = readvec::<String>(&mut s);
    let dominant = &v[1];
    let mut points = 0;
    for _ in 0..4 * v[0].parse::<usize>().unwrap() {
        let mut card = readone::<String>(&mut s);
        let suit = card.pop().unwrap();
        if card == "A" {
            points += 11;
        } else if card == "K" {
            points += 4;
        } else if card == "Q" {
            points += 3;
        } else if card == "T" {
            points += 10;
        } else if card == "J" && dominant.starts_with(suit) {
            points += 20;
        } else if card == "J" {
            points += 2;
        } else if card == "9" && dominant.starts_with(suit) {
            points += 14;
        }
    }
    println!("{}", points);
}
