use puzzles::read::readone;

fn main() {
    let moves = readone::<String>(&mut String::new());
    let mut ball = 1;
    for k in moves.chars() {
        if &k == "A" {
            if ball == 1 {
                ball = 2;
            } else if ball == 2 {
                ball = 1;
            }
        } else if k == "B" {
            if ball == 2 {
                ball = 3;
            } else if ball == 3 {
                ball = 2;
            }
        } else {
            if ball == 1 {
                ball = 3;
            } else if ball == 3 {
                ball = 1;
            }
        }
    }
    println!("{}", ball);
}
