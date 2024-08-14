use puzzles::read::readone;

fn main() {
    let word = readone::<String>(&mut String::new());
    let (_garbage, answer) = word.split_at(word.find('a').unwrap());
    println!("{}", answer);
}
