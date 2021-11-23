use puzzles::read::{readone, readvec};

fn main() {
    let mut s = String::new();
    let v = readvec::<char>(&mut s);
    let dominant = v[1];
    let mut points = 0;
    for _ in 0..4 * v[0] as i32 {
        let mut card = readone::<String>(&mut s);
        let suit = card.pop();
        if card == "A" {
            points += 11;
        } else if card == "K" {
            points += 4;
        } else if card == "Q" {
            points += 3;
        } else if card == "T" {
            points += 10;
        } else if card == "J" && suit == dominant {
            points += 20;
        } else if card == "J" {
            points += 2;
        } else if card == "9" && suit == dominant {
            points += 14;
        }
    }
    println!("{}", points);
}
