use crate::common::{Error, read_non_empty_lines};
use std::num::ParseIntError;

type WorkRange = (i32, i32);
type WorkPair = (WorkRange, WorkRange);

fn parse_line(line: &String) -> Result<WorkPair, Error> {
    let raw_parts: Vec<&str> = line.split(|c| c == ',' || c =='-').collect();
    if raw_parts.len() != 4 {
        Err(Error::General(format!("Invalid line: [{}]", line)))
    }
    else {
        let parts = raw_parts
            .iter()
            .map(|p| p.parse::<i32>())
            .collect::<Result<Vec<i32>, ParseIntError>>()
            .map_err(|e| Error::General(format!("Invalid number: {}", e)))?;

        Ok((
            (parts[0], parts[1]),
            (parts[2], parts[3])
        ))
    }
}

fn is_contained(p1: WorkRange, p2: WorkRange) -> bool {
    p2.0 >= p1.0 && p2.1 <= p1.1
}

fn is_one_contained(pair: WorkPair) -> bool {
    is_contained(pair.0, pair.1) ||
    is_contained(pair.1, pair.0)
}

fn is_overlap(pair: WorkPair) -> bool {
    is_one_contained(pair) ||
    (pair.0.0 >= pair.1.0 && pair.0.0 <= pair.1.1) ||
    (pair.0.1 >= pair.1.0 && pair.0.1 <= pair.1.1)

}

fn do_part_with_condition(input_path: &str, cond: fn(WorkPair) -> bool) -> Result<String, Error> {
    let lines = read_non_empty_lines(input_path)?;

    let work_pairs = (&lines)
        .iter()
        .map(parse_line)
        .collect::<Result<Vec<WorkPair>, Error>>()?;

    let result: i32 = work_pairs
        .iter()
        .map(|pair| cond(*pair) as i32)
        .sum();

    Ok(result.to_string())
}

pub fn part1(input_path: &str) -> Result<String, Error> {
    do_part_with_condition(input_path, is_one_contained)
}

pub fn part2(input_path: &str) -> Result<String, Error> {
    do_part_with_condition(input_path, is_overlap)
}
