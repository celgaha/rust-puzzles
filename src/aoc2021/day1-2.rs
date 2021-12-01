use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let mut readings = Vec::new();
    for _ in 0..2000 {
        let depth = readone::<i32>(&mut s);
        readings.push(depth);
    }
    let mut last = readings[0] + readings[1] + readings[2];
    let mut result = 0;
    for ((a, b), c) in
        (readings.iter().skip(1).zip(readings.iter().skip(2))).zip(readings.iter().skip(3))
    {
        let sum = a + b + c;
        if sum > last {
            result += 1;
        }
        last = sum;
    }
    println!("{}", result);
}
