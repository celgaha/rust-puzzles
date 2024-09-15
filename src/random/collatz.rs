use puzzles::read::readone;

fn main () {
    let mut input = readone::<i32>(&mut String::new());
    while input != 1 {
        if input%2 == 0 {
            input = input/2;
        }
        else {
            input = input*3+1;
        }
        println!("{}", input);
    }
}
