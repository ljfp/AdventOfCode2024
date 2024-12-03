use std::fs::File;
use std::io::{BufReader, BufWriter, BufRead, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);
    let mut reports: Vec<Vec<u8>> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if line.trim().is_empty() { continue; }
        let report: Vec<u8> = line
            .split_whitespace()
            .map(|c| c.parse().expect("Expected a number"))
            .collect();
        reports.push(report);
    }

    let mut safe_count = 0;
    for report in reports.iter() {
        if report.len() < 2 { continue; }

        let mut monotonically_increasing = true;
        let mut monotonically_decreasing = true;
        let mut valid_difference = true;

        for i in 0..(report.len() - 1) {
            let current = report[i];
            let next = report[i + 1];
            let diff = next as i16 - current as i16;

            if diff.abs() < 1 || diff.abs() > 3 {
                valid_difference = false;
                break;
            }

            if diff > 0 {
                monotonically_decreasing = false;
            } else if diff < 0 {
                monotonically_increasing = false;
            } else {
                monotonically_increasing = false;
                monotonically_decreasing = false;
                break;
            }
        }

        if valid_difference & (monotonically_increasing ^ monotonically_decreasing) {
            safe_count += 1;
        }
    }

    let output_file = File::create("output.txt")?;
    let mut writer = BufWriter::new(output_file);
    write!(writer, "{}", safe_count)?;

    Ok(())
}
