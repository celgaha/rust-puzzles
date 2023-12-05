use std::io;

fn main() {
    let mut part = 0;
    let mut last = 0;
    let mut sum = 0;
    let mut first = true;

    for line in io::stdin().lines() {
        let read = line.unwrap();
        let iter = read.chars();
        for k in iter {
            if k.is_ascii_digit() && first == true {
                part += (k as i32 - 48) * 10;
                last = k as i32 - 48;
                first = false;
            } else if k.is_ascii_digit() && first == false {
                last = k as i32 - 48;
            }
        }
        part += last;
        sum += part;
        part = 0;
        last = 0;
        first = true;
    }

    println!("{}", sum);
}
