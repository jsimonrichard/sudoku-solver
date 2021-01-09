pub struct Puzzle(pub Vec<Vec<u8>>);

pub fn print_puzzle(puzzle: &Puzzle) {
    for row in puzzle.0.iter() {
        println!("{}",
            row.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}