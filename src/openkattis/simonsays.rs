use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let k = readone::<i32>(&mut s);
    for _ in 0..k {
        let order = readone::<String>(&mut s);
        if order.starts_with("Simon says") {
            println!("{}", order.chars().skip(10).collect::<String>());
        }
    }
}
