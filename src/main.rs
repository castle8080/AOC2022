use std::time::Instant;
use std::panic::catch_unwind;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

use common::Error;

type ProblemFunction = fn(&str) -> Result<String, Error>;

fn main() {
    let problems: Vec<(&str, &str, &str, ProblemFunction)> = vec![
        ("1", "1", "puzzles/day1-input.txt", day1::part1),
        ("1", "2", "puzzles/day1-input.txt", day1::part2),
        ("2", "1", "puzzles/day2-input.txt", day2::part1),
        ("2", "2", "puzzles/day2-input.txt", day2::part2),
        ("3", "1", "puzzles/day3-input.txt", day3::part1),
        ("3", "2", "puzzles/day3-input.txt", day3::part2),
        ("4", "1", "puzzles/day4-input.txt", day4::part1),
        ("4", "2", "puzzles/day4-input.txt", day4::part2),
        ("5", "1", "puzzles/day5-input.txt", day5::part1),
        ("5", "2", "puzzles/day5-input.txt", day5::part2),
        ("6", "1", "puzzles/day6-input.txt", day6::part1),
        ("6", "2", "puzzles/day6-input.txt", day6::part2),
        ("7", "1", "puzzles/day7-input.txt", day7::part1),
        ("7", "2", "puzzles/day7-input.txt", day7::part2),
        ("8", "1", "puzzles/day8-input.txt", day8::part1),
        ("8", "2", "puzzles/day8-input.txt", day8::part2),
        ("9", "1", "puzzles/day9-input.txt", day9::part1),
        ("9", "2", "puzzles/day9-input.txt", day9::part2),
        ("10", "1", "puzzles/day10-input.txt", day10::part1),
    ];

    println!("Status,Day,Part,Timing,Answer");
    for (day_name, part_name, file_name, p_func) in problems {
        let start = Instant::now();
        let result = catch_unwind(|| { p_func(file_name) });

        match result {
            Ok(Ok(answer)) => {
                println!("OK,{},{},{},{}", day_name, part_name, start.elapsed().as_secs_f64(), answer);
            },
            Ok(Err(e)) => {
                println!("ERROR,{},{},{},{:?}", day_name, part_name, start.elapsed().as_secs_f64(), e);
            },
            Err(_) => {
                println!("ERROR,{},{},{},Panic", day_name, part_name, start.elapsed().as_secs_f64());
            }
        }
    }
}