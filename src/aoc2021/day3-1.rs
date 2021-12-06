use puzzles::binary_to_i32;
use std::io::BufRead;

const NUM_BITS: usize = 12;

fn main() {
    let mut b = [0; NUM_BITS];
    let mut lines = 0;
    let mut values = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let bits = line.unwrap();
        values.push(bits);
        lines += 1;
        for (i, k) in bits.chars().enumerate() {
            if k == '1' {
                b[NUM_BITS-i-1] += 1;
            }
        }
    }
    let mut gamma = [false; NUM_BITS];
    for k in (b.iter()).zip(gamma.iter_mut()) {
        if *k.0 > lines / 2 {
            *k.1 = true;
        }
    }
    let episilon = 2i32.pow(NUM_BITS as u32)-1 - binary_to_i32(&gamma);
    println!("{}", episilon * (binary_to_i32(&gamma)));
}
