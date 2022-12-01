use std::io;

fn main() {
    let mut sums = Vec::new();
    let mut current = 0;

    for line in io::stdin().lines() {
        if let Ok(line) = line.unwrap().parse::<i32>() {
            current += line;
        } else {
            sums.push(current);
            current = 0;
        }
    }
    sums.push(current);

    let result = sums.iter().max().unwrap();
    println!("{}", *result);
}
