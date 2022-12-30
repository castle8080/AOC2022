use crate::common::{Error, read_lines};
use regex::{Captures, Regex};
use lazy_static::lazy_static;

type CargoStacks = Vec<Vec<char>>;

#[derive(Debug)]
struct MoveOperation {
    n: u32,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Input {
    cargo_stacks: CargoStacks,
    operations: Vec<MoveOperation>,
}

lazy_static! {
    static ref STACK_PLACEMENT_RE: Regex = Regex::new(r"^(\s*\[[A-Z]\])+\s*$").unwrap();
}

lazy_static! {
    static ref LABEL_LINE_RE: Regex = Regex::new(r"^(\s+\d)+\s*$").unwrap();
}

lazy_static! {
    //move 1 from 7 to 4
    static ref MOVE_LINE_RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)\s*$").unwrap();
}

fn ensure_n_stacks(cargo_stacks: &mut Vec<Vec<char>>, stack_id: usize) {
    while stack_id >= cargo_stacks.len() {
        cargo_stacks.push(Vec::new())
    }
}

fn add_to_stack(cargo_stacks: &mut Vec<Vec<char>>, c: char, stack_id: usize) {
    ensure_n_stacks(cargo_stacks, stack_id);
    cargo_stacks[stack_id].push(c);
}

fn parse_stack_line(cargo_stacks: &mut Vec<Vec<char>>, line: &String) -> Result<(), Error> {
    for (idx, c) in line.chars().enumerate() {
        if c >= 'A' && c <= 'Z' {
            let stack = idx / 4;
            add_to_stack(cargo_stacks, c, stack);
        }
    }
    Ok(())
}

fn parse_stack_label_line(cargo_stacks: &mut Vec<Vec<char>>, line: &String) -> Result<Vec<String>, Error> {
    let mut labels: Vec<String> = Vec::new();

    for part in line.split(" ") {
        if part.len() > 0 {
            labels.push(String::from(part));
        }
    }

    ensure_n_stacks(cargo_stacks, labels.len());

    Ok(labels)
}

fn parse_move_line(operations: &mut Vec<MoveOperation>, c: &Captures) {
    operations.push(MoveOperation {
        n: c[1].parse().unwrap(),
        from: c[2].parse().unwrap(),
        to: c[3].parse().unwrap()
    })
}

fn reverse_stacks(cargo_stacks: &mut Vec<Vec<char>>) {
    for s in cargo_stacks.iter_mut() {
        s.reverse();
    }
}

fn parse_input(lines: &Vec<String>) -> Result<Input, Error> {
    let mut cargo_stacks: Vec<Vec<char>> = Vec::new();
    let mut operations: Vec<MoveOperation> = Vec::new();

    for line in lines {
        if STACK_PLACEMENT_RE.is_match(line) {
            parse_stack_line(&mut cargo_stacks, line)?;
        }
        else if LABEL_LINE_RE.is_match(line) {
            parse_stack_label_line(&mut cargo_stacks, line)?;
        }
        else if line.len() == 0 {
            // skip
        }
        else {
            match MOVE_LINE_RE.captures(line) {
                None => {
                    return Err(Error::General(format!("Invalid line: [{}]", line)));
                },
                Some(c) => {
                    parse_move_line(&mut operations, &c);
                }
            }
        }
    }

    reverse_stacks(&mut cargo_stacks);

    Ok(Input { cargo_stacks, operations })
}

fn move_stack(cargo_stacks: &mut Vec<Vec<char>>, op: &MoveOperation) {
    let n = (op.n as usize).min(cargo_stacks[op.from - 1].len());

    for _ in 0..n {
        let c = cargo_stacks[op.from - 1].pop().unwrap();
        cargo_stacks[op.to - 1].push(c);
    }
}

pub fn part1(input_path: &str) -> Result<String, Error> {
    let lines = read_lines(input_path)?;
    let mut input = parse_input(&lines)?;

    for move_op in &input.operations {
        move_stack(&mut input.cargo_stacks, move_op)
    }

    let mut msg = String::new();

    for stack in input.cargo_stacks {
        if stack.len() > 0 {
            msg.push(stack.last().map_or(' ', |c| *c));
        }
    }

    Ok(msg)
}