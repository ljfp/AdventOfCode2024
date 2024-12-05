use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        grid.push(line.chars().collect());
    }

    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut count = 0;
    for i in 1..nrows - 1 {
        for j in 1..ncols - 1 {
            if grid[i][j] == 'A' {
                if check_diagonals(&grid, i, j) {
                    count += 1;
                }
            }
        }
    }

    let mut output_file = File::create("output.txt")?;
    write!(output_file, "{}", count)?;

    Ok(())
}

fn check_diagonals(grid: &[Vec<char>], i: usize, j: usize) -> bool {
    let nrows = grid.len();
    let ncols = grid[0].len();

    // Positions for the first diagonal (Top-Left to Bottom-Right)
    let (i1_tl, j1_tl) = (i - 1, j - 1);
    let (i2_br, j2_br) = (i + 1, j + 1);

    // Positions for the second diagonal (Top-Right to Bottom-Left)
    let (i1_tr, j1_tr) = (i - 1, j + 1);
    let (i2_bl, j2_bl) = (i + 1, j - 1);

    if i1_tl >= nrows || j1_tl >= ncols || i2_br >= nrows || j2_br >= ncols {
        return false;
    }
    if i1_tr >= nrows || j1_tr >= ncols || i2_bl >= nrows || j2_bl >= ncols {
        return false;
    }

    let diag1_chars = [grid[i1_tl][j1_tl], grid[i][j], grid[i2_br][j2_br]];
    let diag2_chars = [grid[i1_tr][j1_tr], grid[i][j], grid[i2_bl][j2_bl]];

    let mas = ['M', 'A', 'S'];
    let sam = ['S', 'A', 'M'];

    let diag1_match = diag1_chars == mas || diag1_chars == sam;
    let diag2_match = diag2_chars == mas || diag2_chars == sam;

    diag1_match && diag2_match
}

