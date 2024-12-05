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

    let word = "XMAS";
    let nrows = grid.len();
    let ncols = grid[0].len();
    let directions = [
        (0, 1),    // Right
        (0, -1),   // Left
        (1, 0),    // Down
        (-1, 0),   // Up
        (1, 1),    // Down-Right
        (1, -1),   // Down-Left
        (-1, 1),   // Up-Right
        (-1, -1),  // Up-Left
    ];

    let mut count = 0;
    for i in 0..nrows {
        for j in 0..ncols {
            for &(dx, dy) in &directions {
                if check_word(&grid, i as isize, j as isize, dx, dy, word) {
                    count += 1;
                }
            }
        }
    }

    let mut output_file = File::create("output.txt")?;
    write!(output_file, "{}", count)?;

    Ok(())
}

fn check_word(grid: &[Vec<char>], x: isize, y: isize, dx: isize, dy: isize, word: &str) -> bool {
    let nrows = grid.len() as isize;
    let ncols = grid[0].len() as isize;
    let mut cx = x;
    let mut cy = y;

    for ch in word.chars() {
        if cx < 0 || cy < 0 || cx >= nrows || cy >= ncols {
            return false;
        }
        if grid[cx as usize][cy as usize] != ch {
            return false;
        }
        cx += dx;
        cy += dy;
    }
    true
}

