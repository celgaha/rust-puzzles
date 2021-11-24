use puzzles::read::readone;

fn main() {
    let date = readone::<String>(&mut String::new());
    if date == "OCT 31" || date == "DEC 25" {
        println!("yup");
    } else {
        println!("nope");
    }
}
