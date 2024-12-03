use std::fs::File;
use std::io::{BufReader, BufWriter, BufRead, Write};
use std::error::Error;

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

    left_list.sort();
    right_list.sort();
    let mut dist_list = Vec::new();
    for (left_num, right_num) in left_list.iter().zip(right_list.iter()) {
        let dist = (right_num - left_num).abs();
        dist_list.push(dist);
    }

    let sum: i32 = dist_list.iter().sum();
    let output_file = File::create("output.txt")?;
    let mut writer = BufWriter::new(output_file);
    writeln!(writer, "{}", sum)?;

    /*
    let output_file = File::create("output.txt")?;
    let mut writer = BufWriter::new(output_file);
    for (left_num, right_num) in left_list.iter().zip(right_list.iter()) {
        writeln!(writer, "{}   {}", left_num, right_num)?;
    }
    */
    Ok(())
}

