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
}

#[derive(Debug)]
struct ProgramState {
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
}

pub fn part1(input_path: &str) -> Result<String, Error> {
    let lines = read_non_empty_lines(input_path)?;

    let mut found_x: Vec<(i32, i32)> = Vec::new();
    let interesting_cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let mut st = ProgramState::new();
    for line in lines {
        let cmd = Command::from(line.as_str())?;
        let next_st = st.execute(&cmd);

        for i_cycle in &interesting_cycles {
            if *i_cycle >= st.cycle && *i_cycle < next_st.cycle {
                found_x.push((*i_cycle, st.x));
            } 
        }
        st = next_st;
    }

    let result: i32 = found_x.iter().map(|(cycle, x)| cycle * x ).sum();
    Ok(result.to_string())
}