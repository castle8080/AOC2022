use std::fs::read_to_string;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    InvalidInput,
    IOError
}

fn read_lines(input_path: &str) -> Result<Vec<String>, Error> {
    match read_to_string(input_path) {
        Err(_) => Err(Error::IOError),
        Ok(s) => {
            Ok(s.replace("\r", "")
                .split("\n")
                .into_iter()
                .filter(|s| s.len() > 0)
                .map(|s| s.to_string())
                .collect()
            )
        }
    }
}

fn get_line_parts(line: &String) -> (String, String) {
    let cs: Vec<char> = line.chars().collect();

    let s1 = String::from_iter(&cs[0..cs.len() / 2]);
    let s2 = String::from_iter(&cs[cs.len() / 2 ..]);

    (s1, s2)
}

fn get_priority(c: char) -> i32 {
    if c >= 'a' && c <= 'z' {
        (c as i32) - ('a' as i32) + 1
    }
    else if c >= 'A' && c <= 'Z' {
        (c as i32) - ('A' as i32) + 27
    }
    else {
        -1
    }
}

fn get_priorities(s: String) -> HashSet<i32> {
    let mut priorities = HashSet::new() as HashSet<i32>;
    for p in s.chars().map(get_priority).filter(|c| c >= &0) {
        priorities.insert(p);
    }
    priorities
}

fn get_common_priority(is1: &HashSet<i32>, is2: &HashSet<i32>) -> Option<i32> {
    let samep: Vec<i32> = is1.intersection(is2).map(|i| *i).collect();

    if samep.len() == 1 {
        Some(*samep.get(0).unwrap())
    }
    else {
        None
    }
}

pub fn part1(input_path: &str) -> Result<(), Error> {
    let lines = read_lines(input_path)?;
    let common_p =
        (&lines)
        .iter()
        .map(get_line_parts)
        .map(|(p1, p2)| (get_priorities(p1), get_priorities(p2)))
        .map(|(s1, s2)| get_common_priority(&s1, &s2));

    let result: i32 = common_p.map(|op| op.unwrap()).sum();
    println!("Part 1: {}", result);

    Ok(())
}

fn lines_to_line_groups(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut line_groups: Vec<Vec<String>> = Vec::new();
    let mut line_group: Vec<String> = Vec::new();

    for line in lines {
        line_group.push(line);
        if line_group.len() == 3 {
            line_groups.push(line_group);
            line_group = Vec::new();
        }
    }

    if line_group.len() == 3 {
        line_groups.push(line_group);
    }

    line_groups
}

fn get_label_for_line_group(line_group: &Vec<String>) -> Option<char> {

    let mut char_to_holder: HashMap<char, HashSet<usize>> = HashMap::new();

    for (idx, line) in line_group.iter().enumerate() {
        for c in line.chars() {
            match char_to_holder.get_mut(&c) {
                None => {
                    let mut items: HashSet<usize> = HashSet::new();
                    items.insert(idx);
                    char_to_holder.insert(c, items);
                },
                Some(items) => {
                    items.insert(idx);
                }
            }
        }
    }

    let matching_labels: Vec<char> = char_to_holder
        .iter()
        .filter(|(c, owners)| owners.len() == line_group.len())
        .map(|(c, _)| *c)
        .collect();
        
    if matching_labels.len() == 1 {
        Some(*matching_labels.get(0).unwrap())
    }
    else {
        None
    }

}

pub fn part2(input_path: &str) -> Result<(), Error> {
    let line_groups = lines_to_line_groups(read_lines(input_path)?);

    let result: i32 = line_groups
        .iter()
        .map(|line_group| get_label_for_line_group(line_group).unwrap())
        .map(get_priority)
        .sum();
        
    println!("Part 2: {}", result);

    Ok(())
}

pub fn run() {
    let input_path = "puzzles/day3-1-input.txt";
    println!("Running day 3");
    part1(input_path).unwrap();
    part2(input_path).unwrap();
}