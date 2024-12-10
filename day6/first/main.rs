use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        map.push(line.chars().collect());
    }

    let rows = map.len() as isize;
    let cols = map[0].len() as isize;

    // I fucked up the coordinates here.
    // (0,0) is the top left corner. x is rows, y is columns.
    let (mut x, mut y, mut direction) = (0, 0, Direction::Up);
    for i in 0..rows {
        for j in 0..cols {
            if map[i as usize][j as usize] == '^' {
                x = i;
                y = j;
                break;
            }
        }
    }

    let mut visited = HashSet::new();
    visited.insert((x, y));

    while x >= 0 && x < rows && y >= 0 && y < cols {
        let (dx, dy) = direction.walk();
        let next_x = x + dx;
        let next_y = y + dy;

        if next_x >= 0 && next_x < rows && next_y >= 0 && next_y < cols {
            if map[next_x as usize][next_y as usize] != '#' {
                x = next_x;
                y = next_y;
                visited.insert((x, y));
            } else if map[next_x as usize][next_y as usize] == '#' {
                direction = direction.next();
            }
        } else {
            break;
        }
    }

    let mut output_file = File::create("output.txt")?;
    writeln!(output_file, "{}", visited.len())?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn walk(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}
