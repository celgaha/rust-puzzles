use puzzles::read::readone;

fn main() {
    let mut s = String::new();
    let times = readone::<i32>(&mut s);
    for _ in 0..times {
        let s1 = readone::<String>(&mut s);
        let s2 = readone::<String>(&mut s);
        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();
        let mut compare = String::new();
        for k in 0..s1.len() {
            if b1[k] == b2[k] {
                compare.push('.');
            } else {
                compare.push('*');
            }
        }
        println!("{}", s1);
        println!("{}", s2);
        println!("{}", compare);
        println!("");
    }
}
