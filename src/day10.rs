use crate::common::{Error, read_non_empty_lines};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
enum Command {
    Noop,
    Addx(i32),
}

impl Command {
    fn from(line: &str) -> Result<Command, Error> {
        lazy_static! { static ref REGEX: Regex = Regex::new(r"(addx)\s+(-?\d+)|(noop)").unwrap(); }

        match REGEX.captures(line) {
            None => {
                Err(Error::General(format!("xxx Invalid line: [{}]", line)))
            },
            Some(c) => {
                if c.get(1).is_some() {
                    Ok(Command::Addx(c.get(2).unwrap().as_str().parse::<i32>().unwrap()))
                }
                else if c.get(3).is_some() {
                    Ok(Command::Noop)
                }
                else {
                    Err(Error::General(format!("Invalid line: [{}]", line)))
                }
            }
        }
    }

    fn get_cycle_count(self: &Command) -> i32 {
        match self {
            Command::Noop => 1,
            Command::Addx(_) => 2
        }
    }

    fn read_all_from(input_path: &str) -> Result<Vec<Command>, Error> {
        let lines = read_non_empty_lines(input_path)?;
        lines.iter().map(|s| Command::from(s.as_str())).collect::<Result<Vec<Command>, Error>>()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ProgramState {
    x: i32,
    cycle: i32,
}

impl ProgramState {
    fn new() -> ProgramState {
        ProgramState { x: 1, cycle: 1 }
    }

    fn execute(self: &ProgramState, cmd: &Command) -> ProgramState {
        match cmd {
            Command::Noop => ProgramState {
                x: self.x,
                cycle: self.cycle + cmd.get_cycle_count(),
            },
            Command::Addx(amount) => ProgramState {
                x: self.x + amount,
                cycle: self.cycle + cmd.get_cycle_count(),
            }
        }
    }

    fn run_all(commands: Vec<Command>) -> Vec<(ProgramState, ProgramState)> {
        let mut program_states: Vec<(ProgramState, ProgramState)> = Vec::new();
        let mut st = ProgramState::new();
        for cmd in commands {
            let next_st = st.execute(&cmd);
            program_states.push((st, next_st));
            st = next_st;
        }
        program_states
    }
}

mod pixel {
    use crate::day10::ProgramState;
    use std::collections::HashSet;

    pub fn from_cycle(cycle: i32) -> (i32, i32) {
        let x = (cycle - 1) % 40;
        let y = (cycle - 1) / 40;
        (x, y)
    }

    pub fn get_pixels(program_states: &Vec<(ProgramState, ProgramState)>) -> HashSet<(i32, i32)> {
        let mut pixels: HashSet<(i32, i32)> = HashSet::new();
    
        for (st, next_st) in program_states {
            for cycle in st.cycle..next_st.cycle {
                let (x, y) = from_cycle(cycle);
                for delta in -1..2 {
                    let sprite_x = st.x + delta;
                    if sprite_x == x {
                        pixels.insert((x, y));
                    }
                }
    
            }
        }
    
        pixels
    }

    pub fn render(pixels: &HashSet<(i32, i32)>) -> String {
        let mut content: String = String::new();

        for y in 0..6 {
            for x in 0..40 {
                if pixels.contains(&(x, y)) {
                    content.push('#');
                }
                else {
                    content.push('.');
                }
            }
            content.push('\n');
        }

        content
    }
}

pub fn part1(input_path: &str) -> Result<String, Error> {
    let commands = Command::read_all_from(input_path)?;
    let program_states = ProgramState::run_all(commands);

    let mut found_x: Vec<(i32, i32)> = Vec::new();
    let interesting_cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];

    for (st, next_st) in program_states {
        for i_cycle in &interesting_cycles {
            if *i_cycle >= st.cycle && *i_cycle < next_st.cycle {
                found_x.push((*i_cycle, st.x));
            } 
        }
    }

    let result: i32 = found_x.iter().map(|(cycle, x)| cycle * x ).sum();
    Ok(result.to_string())
}

pub fn part2(input_path: &str) -> Result<String, Error> {
    let commands = Command::read_all_from(input_path)?;
    let program_states = ProgramState::run_all(commands);
    let pixels = pixel::get_pixels(&program_states);
    let result = pixel::render(&pixels);

    println!("{}", result);

    Ok(String::from("<Set output>"))
}