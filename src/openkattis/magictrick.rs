use puzzles::read::readvec;

fn main() {
    let mut s = String::new();
    let v = readvec::<char>(&mut s).sort();
    let mut outcome = 1;
    for k in 0..v.len() - 1 {
        if v[k] == v[k + 1] {
            outcome = 0;
        }
    }
    println!("{}", outcome);
}
