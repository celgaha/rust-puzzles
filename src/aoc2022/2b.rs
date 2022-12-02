use std::io;

fn main() {
    let mut result = 0;
    for line in io::stdin().lines() {
        let full_line = line.unwrap();
        let input = full_line.split(' ').collect::<Vec<_>>();
        if input[0] == "A" {
            if input[1] == "X" {
                result += 3;
            } else if input[1] == "Y" {
                result += 4;
            } else {
                result += 8;
            }
        } else if input[0] == "B" {
            if input[1] == "X" {
                result += 1;
            } else if input[1] == "Y" {
                result += 5;
            } else {
                result += 9;
            }
        } else {
            if input[1] == "X" {
                result += 2;
            } else if input[1] == "Y" {
                result += 6;
            } else {
                result += 7;
            }
        }
    }
    println!("{}", result);
}
