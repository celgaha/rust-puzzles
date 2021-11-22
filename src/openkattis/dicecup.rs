use puzzles::read::readvec;

fn main() {
    let mut s = String::new();
    let dice = readvec::<f32>(&mut s);
    let average = (dice[0] + dice[1]) / 2.0 + 1.0;
    let extra = ((dice[0] - dice[1]) / 2.0).abs();
    let times: i32 = extra * 2 + 1;
    for k in 0..times {
        println!("{}", average - extra + k);
    }
}
