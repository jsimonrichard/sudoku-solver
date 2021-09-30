use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

use crate::puzzle::Puzzle;

fn parse_line(line: String) -> Vec<u8> {
    /*
    Parse a puzzle String according to the format found at
    http://lipas.uwasa.fi/~timan/sudoku/

    Will also parse puzzles that use "." for empty spaces
    */

    let row:Vec<u8> = line.split(" ") // Split each line
                .filter(|&x| !x.is_empty()) // Remove extra whitespace
                .map(|x| if x=="." {
                    0 // Return 0 for empty
                } else {
                    x.parse().expect("Can't parse") // Return string as u8
                })
                .collect();
    
    if row.len() != 9 {
        panic!("Bad input: 9 columns are required for each row");
    }

    return row;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    /*
    Read the file
    */

    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn puzzle_from_file(filename: &str) -> Puzzle {
    /*
    Read file and parse as puzzle
    */

    // The puzzle variable
    let mut puzzle = Puzzle(vec!());

    // Read file lines
    if let Ok(lines) = read_lines(filename) { // Get itorater
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line != "" {
                    puzzle.0.push(
                        parse_line(line)
                    );
                }
            }
        }
    }

    if puzzle.0.len() != 9 {
        panic!("Bad input: 9 rows are required");
    }

    return puzzle;
}

pub fn puzzle_from_stdin() -> Puzzle {
    /*
    Reads puzzle from stdin
    */
    
    // The puzzle variable
    let mut puzzle = Puzzle(vec!());

    // Read stdin
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        if line != "" {
            puzzle.0.push(
                parse_line(line)
            );
        }
    }

    if puzzle.0.len() != 9 {
        panic!("Bad input: 9 rows are required");
    }

    return puzzle;
}