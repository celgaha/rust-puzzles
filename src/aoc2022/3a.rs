use std::io;

fn main() {
    let mut result = 0;
    let mut value;
    for line in io::stdin().lines() {
        let full_line = line.unwrap();
        let count = full_line.chars().count();
        let (front, back) = full_line.split_at(count / 2);
        for k in front.chars() {
            for n in back.chars() {
                if k == n {
                    if n.is_uppercase() {
                        value = (n as i32) - 38;
                    } else {
                        value = (n as i32) - 96;
                    }
                    result += value;
                    break
                }
            }
        }
    }
    println!("{}", result);
}
