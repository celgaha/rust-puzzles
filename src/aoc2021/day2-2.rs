use puzzles::read::readvec;

fn main() {
    let mut s = String::new();
    let mut forward = 0;
    let mut aim = 0;
    let mut depth = 0;
    for _ in 0..1000 {
        let m = readvec::<String>(&mut s);
        if m[0] == "forward" {
            forward += m[1].parse::<i32>().unwrap();
            depth += m[1].parse::<i32>().unwrap() * aim;
        } else if m[0] == "up" {
            aim -= m[1].parse::<i32>().unwrap();
        } else {
            aim += m[1].parse::<i32>().unwrap();
        }
    }
    println!("{}", forward * depth);
}
