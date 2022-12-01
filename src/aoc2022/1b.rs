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

    sums.sort();
    sums.reverse();

    let result = sums[0] + sums[1] + sums[2];

    println!("{}", result);
}
