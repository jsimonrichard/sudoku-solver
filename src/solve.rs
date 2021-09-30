use crate::puzzle::Puzzle;

fn check(i:usize, j:usize, n:usize, puzzle: &Puzzle) -> bool {
    /*
    For cell (i,j), check if the number n will work
    */

    // Check horizontal and vertical at the same time
    for k in 0..9 {
        if puzzle.0[i][k] == n as u8 || puzzle.0[k][j] == n as u8 {
            return false;
        }
    }

    // top left of square cell group
    let group_top_left_i = (i / 3)*3;
    let group_top_left_j = (j / 3)*3;

    // Loop through square cell group
    for group_i in group_top_left_i..group_top_left_i+3 {
        for group_j in group_top_left_j..group_top_left_j+3 {

            if puzzle.0[group_i][group_j] == n as u8 {
                return false;
            }

        }
    }

    // Hasn't failed, so n will work
    return true;
}

fn is_finished(puzzle: &Puzzle) -> bool {
    /*
    Check if the entire puzzle is complete
    */

    for i in 0..9 {
        for j in 0..9 {
            if puzzle.0[i][j] == 0 {
                return false;
            }
        }
    }

    return true;
}

pub fn solve(mut puzzle: Puzzle) -> Puzzle {
    /*
    Solves the puzzle recursively (but not particularly quickly)
    */

    // Loop through the cells
    for i in 0..9 {
        for j in 0..9 {

            // If the cell hasn't been filled in...
            if puzzle.0[i][j]==0 {

                // Loop through all of the possibilities
                for n in 1..10 {
                    if check(i, j, n, &puzzle) { // if the number works
                        puzzle.0[i][j]=n as u8; // Set the cell

                        // Solve the rest of the puzzle using the guess set above
                        // Note: this is recursion
                        puzzle = solve(puzzle);

                        // If the puzzle is complete, reture the puzzle
                        if is_finished(&puzzle) {
                            return puzzle
                        }

                        // Otherwise, reset and try another guess
                        puzzle.0[i][j]=0; // Reset the cell
                    }
                }
                
                return puzzle
            }
        }
    }

    return puzzle;
}