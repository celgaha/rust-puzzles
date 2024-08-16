use puzzles::read::{readtriple, readvec};

fn main() {
    readtriple::<usize>(&mut String::new());
    let mut bplaces = readvec::<i32>(&mut String::new());
    bplaces.sort();
    let mut rplaces = readvec::<i32>(&mut String::new());
    rplaces.sort();
    let mut l = 0;
    let mut failed = 0;
    for k in bplaces {
        while l < rplaces.len() && rplaces[l] <= k - 2 {
            l += 1;
        }
        if l < rplaces.len() && (rplaces[l] == k - 1 || rplaces[l] == k || rplaces[l] == k + 1) {
            l += 1;
        } else {
            failed += 1;
        }
    }
    println!("{}", failed);
}
