use puzzles::read::readvec;

fn main() {
    let numbers = readvec::<String>(&mut String::new());
    let revone = numbers[0]
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
        .unwrap();
    let revtwo = numbers[1]
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
        .unwrap();
    if revone > revtwo {
        println!("{}", revone);
    } else {
        println!("{}", revtwo);
    }
}
