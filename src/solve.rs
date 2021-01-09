use crate::puzzle::Puzzle;

fn check(i:usize,j:usize,n:usize, puzzle: &Puzzle) -> bool {
    for k in 0..9 { // Check horizontal/vertical
        if puzzle.0[i][k] == n as u8 || puzzle.0[k][j] == n as u8 {
            return false;
        }
    }

    // top left of big cell
    let I = (i / 3)*3;
    let J = (j / 3)*3;
    // Loop through cell
    for ihat in I..I+3 {
        for jhat in J..J+3 {
            if puzzle.0[ihat][jhat] == n as u8 {
                return false;
            }
        }
    }

    // Hasn't failed, so n will work
    return true;
}

fn is_finished(puzzle: &Puzzle) -> bool {
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
    // Loop through cells
    for i in 0..9 {
        for j in 0..9 {
            if puzzle.0[i][j]==0 { // Not filled in yet 
                for n in 1..10 { // Loop through all of the possibilities
                    if check(i,j,n,&puzzle) { // if the number works
                        puzzle.0[i][j]=n as u8; // Set the cell

                        puzzle = solve(puzzle);
                        if is_finished(&puzzle) {
                            return puzzle
                        }

                        puzzle.0[i][j]=0; // Reset the cell
                    }
                }
                
                return puzzle
            }
        }
    }

    return puzzle;
}