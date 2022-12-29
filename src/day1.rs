use std::fs;
use std::fmt::format;
use crate::common::{read_lines, Error};

struct Elve {
    id: i32,
    calorie_total: i32
}

fn parse_calorie_lists(path: &str) -> Result<Vec<Vec<i32>>, Error> {
    let mut calories: Vec<i32> = Vec::new();
    let mut elve_items: Vec<Vec<i32>> = Vec::new();

    for line in read_lines(path)? {
        if line == "" {
            if calories.len() > 0 {
                elve_items.push(calories);
                calories = Vec::new();
            }
        }
        else {
            match line.parse::<i32>() {
                Err(e) => return Err(Error::General(format!("Invalid line: {}", e))),
                Ok(calorie) => calories.push(calorie)
            }
        }
    }

    if calories.len() > 0 {
        elve_items.push(calories);
    }

    Ok(elve_items)
}

fn to_elves(elve_items: Vec<Vec<i32>>) -> Vec<Elve> {
    elve_items
        .iter()
        .enumerate()
        .map(|(i, items)| Elve { id: i as i32, calorie_total: items.iter().sum() })
        .collect()
}

pub fn part1(path: &str) -> Result<String, Error> {
    let elve_items = parse_calorie_lists(path)?;
    let elves = to_elves(elve_items);

    let elve = elves
        .iter()
        .max_by_key(|e| e.calorie_total)
        .expect("There was less than one.");

    Ok(elve.calorie_total.to_string())
}

pub fn part2(path: &str) -> Result<String, Error> {
    let elve_items = parse_calorie_lists(path)?;
    let mut elves = to_elves(elve_items);

    elves.sort_by_key(|e| e.calorie_total * -1);
    let top3_sum: i32 = elves
        .iter()
        .take(3)
        .map(|e| e.calorie_total)
        .sum();

    Ok(top3_sum.to_string())
}

pub fn run() {
    let input_path = "puzzles/day1-1-input.txt";
    println!("Running day 1:");
    let r1 = part1(input_path);
    println!("Part 1: {:?}", r1);

    let r2 = part2(input_path);
    println!("Part 2: {:?}", r2);
}