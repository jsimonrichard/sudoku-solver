use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

use crate::puzzle::Puzzle;

fn parse_line(line: String) -> Vec<u8> {
    line.split(" ")
        .filter(|&x| !x.is_empty())
        .map(|x| if x=="." {
            0
        } else {
            x.parse().expect("Can't parse")
        })
        .collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn puzzle_from_file(filename: &str) -> Puzzle {
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

    return puzzle;
}

pub fn puzzle_from_stdin() -> Puzzle {
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

    return puzzle;
}