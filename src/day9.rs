use crate::common::{Error, read_non_empty_lines};
use regex::Regex;
use std::collections::HashSet;
use lazy_static::lazy_static;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn from(s: &str) -> Result<Direction, Error> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(Error::General(format!("Invalid direction: [{}]", s)))
        }
    }

    fn get_movement_vector(self: &Direction) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}


#[derive(Debug)]
struct Movement {
    direction: Direction,
    amount: i32,
}

impl Movement {
    fn from(line: &str) -> Result<Movement, Error> {
        lazy_static! { static ref LINE_RE: Regex = Regex::new(r"([UDLR]) (\d+)").unwrap(); }
        match LINE_RE.captures(line) {
            None => Err(Error::General(format!("Invalid line: [{}]", line))),
            Some(cap) => {
                Ok(Movement {
                    direction: Direction::from(cap.get(1).unwrap().as_str())?,
                    amount: cap.get(2).unwrap().as_str().parse::<i32>().unwrap()
                })
            }
        }
    }

    fn parse_input(input_path: &str) -> Result<Vec<Movement>, Error> {
        let lines = read_non_empty_lines(input_path)?;
    
        let mut movements: Vec<Movement> = Vec::new();
        for line in lines {
            movements.push(Movement::from(line.as_str())?);
        }
    
        Ok(movements)
    }   
}

struct Part1Process {
    head_position: (i32, i32),
    tail_position: (i32, i32),
    tail_visited: HashSet<(i32, i32)>,
}

impl Part1Process {
    fn new() -> Part1Process {
        let mut _self = Part1Process {
            head_position: (0, 0),
            tail_position: (0, 0),
            tail_visited: HashSet::new()
        };
        _self.tail_visited.insert(_self.tail_position);
        _self
    }

    fn process_movements(self: &mut Part1Process, movements: &Vec<Movement>) {
        for movement in movements {
            self.mv(movement);
        }
    }

    fn mv(self: &mut Part1Process, movement: &Movement) {
        let move_vec = movement.direction.get_movement_vector();

        for _ in 0..movement.amount {
            let new_head = (self.head_position.0 + move_vec.0, self.head_position.1 + move_vec.1);
            self.head_position = new_head;

            let xd = self.head_position.0 - self.tail_position.0;
            let yd = self.head_position.1 - self.tail_position.1;

            if xd.abs() > 1 || yd.abs() > 1 {
                let new_tail = (self.tail_position.0 + xd.signum(), self.tail_position.1 + yd.signum());
                self.tail_position = new_tail;
                self.tail_visited.insert(self.tail_position);
            }
        }
    }
}

pub fn part1(input_path: &str) -> Result<String, Error> {
    let movements = Movement::parse_input(input_path)?;
    let mut process = Part1Process::new();
    process.process_movements(&movements);

    let result = process.tail_visited.len();
    Ok(result.to_string())
}