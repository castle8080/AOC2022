use crate::common::{Error, read_non_empty_lines};

use regex::Regex;
use lazy_static::lazy_static;

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
        _ => Err(Error::General(format!("Invalid character for game result: {}", c)))
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
        _ => Err(Error::General(format!("Invalid character for game choice: {}", c)))
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
        None => Err(Error::General(format!("Invalid input line: [{}]", line)))
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

fn parse_input(path: &str, line_parser: GameChoiceLineParser) -> Result<Vec<(GameChoice, GameChoice)>, Error> {
    let lines = read_non_empty_lines(path)?;
    let mut all_choices: Vec<(GameChoice, GameChoice)> = Vec::new();

    for line in lines {
        all_choices.push(line_parser(line)?)
    }

    Ok(all_choices)
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

fn run_part(path: &str, line_parser: GameChoiceLineParser) -> Result<String, Error> {
    let all_choices = parse_input(path, line_parser)?;

    let score = all_choices
        .iter()
        .map(|(c1, c2)| calculate_score(c2, c1))
        .sum::<i32>();

    Ok(score.to_string())
}

pub fn part1(path: &str) -> Result<String, Error> {
    run_part(path, parse_line_p1)
}

pub fn part2(path: &str) -> Result<String, Error> {
    run_part(path, parse_line_p2)
}
