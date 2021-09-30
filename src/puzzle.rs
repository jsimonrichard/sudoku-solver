// Puzzle is 2D array of bytes (we only need the values 0-9)
// 0 means a cell has not been filled in
pub struct Puzzle(pub Vec<Vec<u8>>);

pub fn print_puzzle(puzzle: &Puzzle) {
    /*
    Display the puzzle
    */

    for row in puzzle.0.iter() {

        // Print each row
        println!("{}",
            row.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );

    }
}