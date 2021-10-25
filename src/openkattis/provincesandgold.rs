use puzzles::read::readmore;

fn main() {
    let mut s = String::new();
    let mut treasure = "Copper";
    let mut victory = "Estate";
    let points: i32 = readmore::<i32>(&mut s)
        .enumerate()
        .map(|(a, card)| card * (3 - a as i32))
        .sum();
    if points >= 3 && points < 6 {
        treasure = "Silver";
    } else if points >= 6 {
        treasure = "Gold";
    }
    if points >= 5 && points < 8 {
        victory = "Duchy";
    } else if points >= 8 {
        victory = "Province";
    }
    if points <= 1 {
        println!("{}", treasure);
    } else {
        println!("{} or {}", victory, treasure);
    }
}
