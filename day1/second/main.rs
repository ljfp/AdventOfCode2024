use std::fs::File;
use std::io::{BufReader, BufWriter, BufRead, Write};
use std::error::Error;
use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            let left_num = parts[0].parse::<i32>()?;
            let right_num = parts[1].parse::<i32>()?;
            left_list.push(left_num);
            right_list.push(right_num);
        }
    }


    let left_set: HashSet<i32> = left_list.into_iter().collect();
    let mut freq_map: HashMap<i32, i32> = HashMap::new();
    for &key in &left_set { freq_map.insert(key, 0); }

    for idr in right_list.iter() {
        freq_map.entry(*idr).and_modify(|e| *e += 1);
    }

    // Sum all the keys in the freq_map times their values.
    let mut sum = 0;
    for (key, value) in freq_map.iter() {
        sum += key * value;
    }

    // Write the sum to "output.txt" file.
    let output_file = File::create("output.txt")?;
    let mut writer = BufWriter::new(output_file);
    write!(writer, "{}", sum)?;

    Ok(())
}
