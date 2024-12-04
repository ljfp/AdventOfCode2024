use std::fs::File;
use std::io::{Read, Write, BufWriter};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut instructions = String::new();
    File::open("input.txt")?.read_to_string(&mut instructions)?;

    let mut total = 0u32;
    let mut index = 0;
    let mut mul_enabled = true;

    while let Some((instr_type, pos)) = find_next_instruction(&instructions, index) {
        index = pos;
        match instr_type {
            InstructionType::Mul => {
                if let Some((x, y, consumed_chars)) = parse_mul_instruction(&instructions[index..]) {
                    if mul_enabled {
                        total += x * y;
                    }
                    index += consumed_chars;
                } else {
                    index += 3; // Move past "mul" to prevent infinite loop
                }
            }
            InstructionType::Do => {
                mul_enabled = true;
                index += "do()".len();
            }
            InstructionType::Dont => {
                mul_enabled = false;
                index += "don't()".len();
            }
        }
    }

    let mut writer = BufWriter::new(File::create("output.txt")?);
    write!(writer, "{}", total)?;

    Ok(())
}

enum InstructionType {
    Mul,
    Do,
    Dont,
}

fn find_next_instruction(s: &str, index: usize) -> Option<(InstructionType, usize)> {
    let rest = &s[index..];

    let pos_mul = rest.find("mul(").map(|pos| (InstructionType::Mul, pos + index));
    let pos_do = rest.find("do()").map(|pos| (InstructionType::Do, pos + index));
    let pos_dont = rest.find("don't()").map(|pos| (InstructionType::Dont, pos + index));

    let mut positions = vec![];
    if let Some(pos) = pos_mul {
        positions.push(pos);
    }
    if let Some(pos) = pos_do {
        positions.push(pos);
    }
    if let Some(pos) = pos_dont {
        positions.push(pos);
    }

    positions.sort_by_key(|&(_, pos)| pos);
    positions.into_iter().next()
}

fn parse_mul_instruction(s: &str) -> Option<(u32, u32, usize)> {
    if !s.starts_with("mul(") {
        return None;
    }

    let end_pos = s.find(')')?;
    let content = &s[4..end_pos];

    let mut parts = content.split(',');
    let x_str = parts.next()?.trim();
    let y_str = parts.next()?.trim();

    if parts.next().is_some() {
        return None;
    }
    if x_str.len() > 3 || y_str.len() > 3 {
        return None;
    }

    let x: u32 = x_str.parse().ok()?;
    let y: u32 = y_str.parse().ok()?;

    let consumed_chars = end_pos + 1; // +1 to include the ')'
    Some((x, y, consumed_chars))
}

