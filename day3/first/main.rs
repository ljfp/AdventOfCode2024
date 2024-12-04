use std::fs::File;
use std::io::{Read, Write, BufWriter};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut instructions = String::new();
    File::open("input.txt")?.read_to_string(&mut instructions)?;

    let mut total = 0;
    let mut index = 0;

    while let Some(pos) = instructions[index..].find("mul(") {
        index += pos; // Update index to the position of "mul("
        let substr = &instructions[index..];

        if let Some((x, y, consumed_chars)) = parse_mul_instruction(substr) {
            total += x * y;
            index += consumed_chars; // Move index past the parsed instruction
        } else {
            index += 3; // Move index forward to avoid infinite loop
        }
    }

    let mut writer = BufWriter::new(File::create("output.txt")?);
    write!(writer, "{}", total)?;

    Ok(())
}

// Function to parse a valid 'mul(X,Y)' instruction from a string slice
fn parse_mul_instruction(s: &str) -> Option<(u32, u32, usize)> {
    if !s.starts_with("mul(") { return None; }

    let end_pos = s.find(')')?;
    let content = &s[4..end_pos]; // Extract the content between '(' and ')'

    let mut parts = content.split(',');
    let x_str = parts.next()?.trim();
    let y_str = parts.next()?.trim();

    if parts.next().is_some() { return None; }
    if x_str.len() > 3 || y_str.len() > 3 { return None; }

    let x: u32 = x_str.parse().ok()?;
    let y: u32 = y_str.parse().ok()?;

    let consumed_chars = end_pos + 1; // +1 to include the ')'

    Some((x, y, consumed_chars))
}

