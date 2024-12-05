use std::collections::HashMap;
use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut ordering_rules = Vec::new();
    let mut updates = Vec::new();
    let mut parsing_rules = true;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let parts: Vec<&str> = line.split('|').collect();
            let x = parts[0].parse::<u32>()?;
            let y = parts[1].parse::<u32>()?;
            ordering_rules.push((x, y));
        } else {
            let update: Vec<u32> = line.split(',').map(|x| x.parse::<u32>()).collect::<Result<_, _>>()?;
            updates.push(update);
        }
    }

    let mut middle_sum = 0;
    for update in &updates {
        if is_update_ordered(update, &ordering_rules) {
            let middle = find_middle(update);
            middle_sum += middle;
        }
    }

    let mut output_file = File::create("output.txt")?;
    writeln!(output_file, "{}", middle_sum)?;

    Ok(())
}

fn is_update_ordered(update: &[u32], rules: &[(u32, u32)]) -> bool {
    let positions: HashMap<u32, usize> = update
        .iter()
        .enumerate()
        .map(|(i, &page)| (page, i))
        .collect();

    for &(x, y) in rules {
        if let (Some(&pos_x), Some(&pos_y)) = (positions.get(&x), positions.get(&y)) {
            if pos_x > pos_y {
                return false;
            }
        }
    }

    return true;
}

fn find_middle(update: &[u32]) -> u32 {
    let middle_idx = update.len() / 2;
    update[middle_idx]
}

