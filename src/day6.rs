use std::collections::HashMap;

use crate::common::{Error, read_non_empty_lines};

fn get_start_of_packet(chars: Vec<char>, end_packet_len: usize) -> Option<i32> {
    let mut c_count: HashMap<char, usize> = HashMap::new();
    let mut unique_count = 0;

    for idx in 0..chars.len() {
        if idx >= end_packet_len {
            let drop_c = chars[idx - end_packet_len];
            let new_count = c_count.get(&drop_c).unwrap() - 1;
            c_count.insert(drop_c, new_count);
            if new_count == 0 {
                unique_count -= 1;
            }
        }
        {
            let new_c = chars[idx];
            let new_count = c_count.get(&new_c).unwrap_or(&0) + 1;
            c_count.insert(new_c, new_count);
            if new_count == 1 {
                unique_count += 1;
            }

            if unique_count == end_packet_len {
                return Some(idx as i32 + 1);
            }
        }
    }

    None
} 

fn get_input(input_path: &str) -> Result<String, Error> {
    let mut lines = read_non_empty_lines(input_path)?;
    match lines.pop() {
        Some(line) => Ok(line),
        None => Err(Error::General(format!("Invalid input.")))
    }
}

pub fn run_part(input_path: &str, end_packet_len: usize) -> Result<String, Error> {
    let line = get_input(input_path)?;
    let chars: Vec<char> = line.chars().collect();

    match get_start_of_packet(chars, end_packet_len) {
        None => Err(Error::General(format!("Could not find answer."))),
        Some(n) => Ok(n.to_string()) 
    }
}

pub fn part1(input_path: &str) -> Result<String, Error> {
    run_part(input_path, 4)
}

pub fn part2(input_path: &str) -> Result<String, Error> {
    run_part(input_path, 14)
}