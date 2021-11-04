use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let mut name = readone::<String>(&mut s);
    name.retain(|c| c.is_uppercase());
    println!("{}", name);
}
