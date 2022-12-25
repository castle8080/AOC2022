use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
enum Error {
    InvalidInput,
    IOError
}

#[derive(Debug, Clone, Copy)]
enum GameChoice {
    Rock,
    Paper,
    Scissor
}


#[derive(Debug, Clone, Copy)]
enum GameResult {
    Loose,
    Draw,
    Win
}

fn get_game_result(c: char) -> Result<GameResult, Error> {
    match c {
        'X' => Ok(GameResult::Loose),
        'Y' => Ok(GameResult::Draw),
        'Z' => Ok(GameResult::Win),
        _ => Err(Error::InvalidInput)
    }
}

fn get_game_choice(c: char) -> Result<GameChoice, Error> {
    match c {
        'A' => Ok(GameChoice::Rock),
        'B' => Ok(GameChoice::Paper),
        'C' => Ok(GameChoice::Scissor),
        'X' => Ok(GameChoice::Rock),
        'Y' => Ok(GameChoice::Paper),
        'Z' => Ok(GameChoice::Scissor),
        _ => Err(Error::InvalidInput)
    }
}

fn parse_line_char_codes(line: String) -> Result<(char, char), Error> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"^([ABC])\s+([XYZ])").unwrap();
    }

    match LINE_RE.captures(line.as_str()) {
        Some(c) => {
            let c1 = c.get(1).unwrap().as_str().chars().next().unwrap();
            let c2 = c.get(2).unwrap().as_str().chars().next().unwrap();
            return Ok((c1, c2));
        }
        None => Err(Error::InvalidInput)
    }
}

fn parse_line_p1(line: String) -> Result<(GameChoice, GameChoice), Error> {
    let (c1, c2) = parse_line_char_codes(line)?;
    return Ok((get_game_choice(c1)?, get_game_choice(c2)?));
}

fn parse_line_p2(line: String) -> Result<(GameChoice, GameChoice), Error> {
    let (c1, c2) = parse_line_char_codes(line)?;
    let choice1 = get_game_choice(c1)?;
    let result = get_game_result(c2)?;

    Ok(match result {
        GameResult::Draw => (choice1, choice1),
        GameResult::Win => match choice1 {
            GameChoice::Rock => (choice1, GameChoice::Paper),
            GameChoice::Paper => (choice1, GameChoice::Scissor),
            GameChoice::Scissor => (choice1, GameChoice::Rock)
        },
        GameResult::Loose => match choice1 {
            GameChoice::Rock => (choice1, GameChoice::Scissor),
            GameChoice::Paper => (choice1, GameChoice::Rock),
            GameChoice::Scissor => (choice1, GameChoice::Paper)
        }
    })
}

type GameChoiceLineParser = fn(String) -> Result<(GameChoice, GameChoice), Error>;

fn parse_input_from_file(f: File, line_parser: GameChoiceLineParser) -> Result<Vec<(GameChoice, GameChoice)>, Error> {
    let mut reader = BufReader::new(f);
    let mut all_choices: Vec<(GameChoice, GameChoice)> = Vec::new();

    loop {
        let mut line = String::new();
        let n_read = reader.read_line(&mut line).expect("able to read.");
        if n_read == 0 {
            break;
        }
        else {
            match line_parser(line) {
                Ok(values) => all_choices.push(values),
                Err(e) => return Err(e)
            }
        }
    }

    Ok(all_choices)
}

fn parse_input(path: &str, line_parser: GameChoiceLineParser) -> Result<Vec<(GameChoice, GameChoice)>, Error> {
    match File::open(path) {
        Err(_) => Err(Error::IOError),
        Ok(f) => parse_input_from_file(f, line_parser)
    }
}

fn calculate_score(c1: &GameChoice, c2: &GameChoice) -> i32 {
    return match c1 {
        GameChoice::Rock => match c2 {
            GameChoice::Rock => 1 + 3,
            GameChoice::Paper => 1 + 0,
            GameChoice::Scissor => 1 + 6
        },
        GameChoice::Paper => match c2 {
            GameChoice::Rock => 2 + 6,
            GameChoice::Paper => 2 + 3,
            GameChoice::Scissor => 2 + 0
        },
        GameChoice::Scissor => match c2 {
            GameChoice::Rock => 3 + 0,
            GameChoice::Paper => 3 + 6,
            GameChoice::Scissor => 3 + 3
        }
    }
}

fn run_part(name: &str, path: &str, line_parser: GameChoiceLineParser) {
    let all_choices = parse_input(path, line_parser).unwrap();

    let score = all_choices
        .iter()
        .map(|(c1, c2)| calculate_score(c2, c1))
        .sum::<i32>();

    println!("{}: {}", name, score);
}

pub fn part1(path: &str) {
    run_part("Part 1", path, parse_line_p1);
}

pub fn part2(path: &str) {
    run_part("Part 2", path, parse_line_p2);
}

pub fn run() {
    let input_path = "puzzles/day2-1-input.txt";
    println!("Running day 2");
    part1(input_path);
    part2(input_path);
}