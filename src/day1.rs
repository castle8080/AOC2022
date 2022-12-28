use std::fs;

/*
Santa's reindeer typically eat regular reindeer food, but they need a lot of
magical energy to deliver presents on Christmas. For that, their favorite
snack is a special type of star fruit that only grows deep in the jungle.
The Elves have brought you on their annual expedition to the grove where
the fruit grows.

To supply enough magical energy, the expedition needs to retrieve a minimum
of fifty stars by December 25th. Although the Elves assure you that the grove
has plenty of fruit, you decide to grab any fruit you see along the way,
just in case.

Collect stars by solving puzzles. Two puzzles will be made available on each
day in the Advent calendar; the second puzzle is unlocked when you complete
the first. Each puzzle grants one star. Good luck!

The jungle must be too overgrown and difficult to navigate in vehicles or
access from the air; the Elves' expedition traditionally goes on foot. As
your boats approach land, the Elves begin taking inventory of their supplies.
One important consideration is food - in particular, the number of Calories
each Elf is carrying (your puzzle input).

The Elves take turns writing down the number of Calories contained by the
various meals, snacks, rations, etc. that they've brought with them, one item
per line. Each Elf separates their own inventory from the previous Elf's
inventory (if any) by a blank line.

For example, suppose the Elves finish writing their items' Calories and end
up with the following list:

*/

struct Elve {
    id: i32,
    calorie_total: i32
}

fn parse_calorie_lists(path: &str) -> Vec<Vec<i32>> {
    let content = fs::read_to_string(path)
        .expect("Could not read file.")
        .replace("\r", "");

    let mut elve_items: Vec<Vec<i32>> = Vec::new();

    for section in content.split("\n\n") {
        let mut items: Vec<i32> = Vec::new();
        for line in section.split("\n") {
            if line != "" {
                items.push(line.parse::<i32>().unwrap());
            }
        }
        if items.len() > 0 {
            elve_items.push(items);
        }
    }

    elve_items
}

fn to_elves(elve_items: Vec<Vec<i32>>) -> Vec<Elve> {
    elve_items
        .iter()
        .enumerate()
        .map(|(i, items)| Elve { id: i as i32, calorie_total: items.iter().sum() })
        .collect()
}

pub fn part1(path: &str) {
    let elve_items = parse_calorie_lists(path);
    let elves = to_elves(elve_items);

    let elve = elves
        .iter()
        .max_by_key(|e| e.calorie_total)
        .expect("There was less than one.");

    println!("Part 1: {}", elve.calorie_total);
}

pub fn part2(path: &str) {
    let elve_items = parse_calorie_lists(path);
    let mut elves = to_elves(elve_items);

    elves.sort_by_key(|e| e.calorie_total * -1);
    let top3_sum: i32 = elves
        .iter()
        .take(3)
        .map(|e| e.calorie_total)
        .sum();

    println!("Part 2: {}", top3_sum);
}

pub fn run() {
    let input_path = "puzzles/day1-1-input.txt";
    println!("Running day 1:");
    part1(input_path);
    part2(input_path);
}