use clap::{Arg,App};

mod puzzle;
mod parse_puzzle;
mod solve;

fn main() {
    /*
    A rather slow sudoku solver
    */

    // Parse program arguments
    let matches = App::new("SudokuSolver")
        .version("0.1.0")
        .author("J. Simon Richard <jsimonrichard@gmail.com>")
        .about("Solves a sudoku grid lazily (puzzle in the style of the puzzles
                                    at http://lipas.uwasa.fi/~timan/sudoku/)")
        .arg(Arg::with_name("file")
                 .short("f")
                 .long("file")
                 .takes_value(true)
                 .required(false)
                 .help("Get a puzzle from a file"))
        .get_matches();
    
    // Get the puzzle
    let my_puzzle;
    if matches.is_present("file") {
        
        // Get puzzle from file
        my_puzzle = parse_puzzle::puzzle_from_file(
            matches.value_of("file").unwrap()
        ); // Read the contents of the file

    } else {
        
        // Get data from stdin
        my_puzzle = parse_puzzle::puzzle_from_stdin();

    };

    // Solve the puzzle
    let solved_puzzle = solve::solve(my_puzzle);

    // Print the puzzle
    puzzle::print_puzzle(&solved_puzzle);
}
