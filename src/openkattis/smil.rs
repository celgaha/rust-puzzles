use puzzles::read::readone;

fn main() {
    let smiles = readone::<String>(&mut String::new());
    let string = smiles
        .match_indices(":)")
        .chain(smiles.match_indices(";)"))
        .chain(smiles.match_indices(":-)"))
        .chain(smiles.match_indices(";-)"))
        .collect::<Vec<_>>();
    for k in string {
        println!("{}", k.0);
    }
}
