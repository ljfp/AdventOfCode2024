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
            // let middle = find_middle(update);
            // middle_sum += middle;
            continue;
        } else {
            let ordered_update = reorder_update(update, &ordering_rules);
            let middle = find_middle(&ordered_update);
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

fn reorder_update(update: &[u32], rules: &[(u32, u32)]) -> Vec<u32> {
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();

    for &page in update {
        graph.entry(page).or_default();
        in_degree.entry(page).or_insert(0);
    }

    for &(x, y) in rules {
        if update.contains(&x) && update.contains(&y) {
            graph.entry(x).or_default().push(y);
            *in_degree.entry(y).or_insert(0) += 1;
        }
    }

    // Kahn's algorithm for topological sorting.
    // https://en.wikipedia.org/wiki/Topological_sorting#Algorithms
    let mut sorted = Vec::new();
    let mut stack: Vec<u32> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&page, _)| page)
        .collect();

    while let Some(node) = stack.pop() {
        sorted.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(&neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        stack.push(neighbor);
                    }
                }
            }
        }
    }

    return sorted
}
