
use std::time::Instant;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;

use common::Error;

type ProblemFunction = fn(&str) -> Result<String, Error>;

fn main() {

    let problems: Vec<(&str, &str, &str, ProblemFunction)> = vec![
        ("Day1", "Part1", "puzzles/day1-1-input.txt", day1::part1),
        ("Day1", "Part2", "puzzles/day1-1-input.txt", day1::part2),
        ("Day2", "Part1", "puzzles/day2-1-input.txt", day2::part1),
        ("Day2", "Part2", "puzzles/day2-1-input.txt", day2::part2),
        ("Day3", "Part1", "puzzles/day3-1-input.txt", day3::part1),
        ("Day3", "Part2", "puzzles/day3-1-input.txt", day3::part2),
        ("Day4", "Part1", "puzzles/day4-1-input.txt", day4::part1),
        ("Day4", "Part2", "puzzles/day4-1-input.txt", day4::part2),
    ];

    println!("Status,Day,Part,Timing,Answer");
    for (day_name, part_name, file_name, p_func) in problems {
        let start = Instant::now();
        let result = p_func(file_name);
        match result {
            Ok(answer) => {
                println!("OK,{},{},{},{}", day_name, part_name, start.elapsed().as_secs_f64(), answer);
            },
            Err(e) => {
                println!("ERROR,{},{},{},{:?}", day_name, part_name, start.elapsed().as_secs_f64(), e);
            }
        }
    }
}