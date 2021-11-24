use puzzles::read::readone;

fn main() {
    let hey = readone::<String>(&mut String::new());
    let e = (0..(hey.len() - 2) * 2).map(|_| "e").collect::<String>();
    println!("h{}y", e);
}
