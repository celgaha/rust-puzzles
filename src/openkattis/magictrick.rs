use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let mut v = readone::<String>(&mut s).chars().collect::<Vec<_>>();
    v.sort();
    let mut outcome = 1;
    for k in 0..v.len() - 1 {
        if v[k] == v[k + 1] {
            outcome = 0;
        }
    }
    println!("{}", outcome);
}
