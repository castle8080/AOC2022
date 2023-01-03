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

struct RopePositions {
    positions: Vec<(i32, i32)>,
    tail_visited: HashSet<(i32, i32)>,
}

impl RopePositions {
    fn new(knot_size: usize) -> RopePositions {
        let mut _self = RopePositions {
            positions: Vec::new(),
            tail_visited: HashSet::new(),
        };
        for _ in 0..knot_size {
            _self.positions.push((0, 0));
        }
        if _self.positions.len() > 0 {
            _self.tail_visited.insert((0, 0));
        }
        _self
    }

    fn process_movements(self: &mut RopePositions, movements: &Vec<Movement>) {
        for movement in movements {
            self.mv(movement);
        }
    }

    fn mv(self: &mut RopePositions, movement: &Movement) {
        if self.positions.len() == 0 {
            return;
        }

        let move_vec = movement.direction.get_movement_vector();
        for _ in 0..movement.amount {
            let head_pos = self.positions.get(0).unwrap();
            self.positions[0] = (head_pos.0 + move_vec.0, head_pos.1 + move_vec.1);

            for pos in 1..self.positions.len() {
                let tail_pos = &self.positions[pos];
                let head_pos = &self.positions[pos - 1];

                let xd = head_pos.0 - tail_pos.0;
                let yd = head_pos.1 - tail_pos.1;

                if xd.abs() > 1 || yd.abs() > 1 {
                    let new_pos = (tail_pos.0 + xd.signum(), tail_pos.1 + yd.signum());
                    self.positions[pos] = new_pos;
                }
            }

            self.tail_visited.insert(*self.positions.get(self.positions.len() - 1).unwrap());
        }
    }
}

fn run_part(input_path: &str, knots: usize) -> Result<String, Error> {
    let movements = Movement::parse_input(input_path)?;
    let mut rope = RopePositions::new(knots);
    rope.process_movements(&movements);

    let result = rope.tail_visited.len();
    Ok(result.to_string())
}

pub fn part1(input_path: &str) -> Result<String, Error> {
    run_part(input_path, 2)
}

pub fn part2(input_path: &str) -> Result<String, Error> {
    run_part(input_path, 10)
}

