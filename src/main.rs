mod common;
mod day1;
mod day2;
mod day3;

use common::Error;

type ProblemFunction = fn(&str) -> Result<String, Error>;

fn main() {

    let problems: Vec<(&str, &str, &str, ProblemFunction)> = vec![
        ("Day1", "Part1", "puzzles/day1-1-input.txt", day1::part1),
        ("Day1", "Part2", "puzzles/day1-1-input.txt", day1::part2),
        ("Day2", "Part1", "puzzles/day2-1-input.txt", day2::part1),
        ("Day2", "Part2", "puzzles/day2-1-input.txt", day2::part2),
    ];

    for (day_name, part_name, file_name, p_func) in problems {
        let result = p_func(file_name);
        match result {
            Ok(answer) => {
                println!("OK,{},{},{}", day_name, part_name, answer);
            },
            Err(e) => {
                println!("ERROR,{},{},{:?}", day_name, part_name, e);
            }
        }
    }
}
