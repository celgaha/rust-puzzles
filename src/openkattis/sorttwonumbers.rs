use puzzles::util::read::readvec;

pub fn main() {
    let mut s = String::new();
    let v = readvec::<i32>(&mut s);
    let a = v[0];
    let b = v[1];
    if b < a {
        println!("{} {}", b, a)
    } else {
        println!("{} {}", a, b)
    }
}
