use puzzles::read::readvec;

fn main() {
    let mut s = String::new();
    let v = readvec::<i32>(&mut s);
    println!("{}", v[1])
}
