use std::collections::HashMap;

use crate::common::{Error, read_non_empty_lines};

#[derive(Debug)]
pub struct DirectoryNode {
    name: String,
    files: HashMap<String, FileNode>,
    directories: HashMap<String, DirectoryNode>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct FileNode {
    name: String,
    size: u32,
}

impl DirectoryNode {
    fn new(name: String) -> DirectoryNode {
        DirectoryNode {
            name: name,
            files: HashMap::new(),
            directories: HashMap::new(),
        }
    }

    // TODO: I want to write this logic to be mut or non-mut.
    fn get_dir(self: &mut DirectoryNode, path: &Vec<String>) -> Option<&mut DirectoryNode> {
        let mut e = self;
        for name in path {
            match e.directories.get_mut(name) {
                Some(dir_ent) => {
                    e = dir_ent;
                },
                None => {
                    return None;
                }
            }
        }
        Some(e)
    }
}

impl FileNode {
    fn new(name: String, size: u32) -> FileNode {
        FileNode {
            name: name,
            size: size,
        }
    }
}

fn process_cd(root: &mut DirectoryNode, current_path: &mut Vec<String>, d_name: &str)
    -> Result<(), Error>
{
    if d_name == "/" {
        current_path.clear();
    }
    else if d_name == ".." {
        current_path.pop();
    }
    else {
        let current_dir = root.get_dir(current_path).unwrap();
        match current_dir.directories.get(d_name) {
            None => {
                println!("[WARNING] Not sub directory: {}", d_name);
            },
            Some(_) => {
                current_path.push(String::from(d_name));
            }
        }
    }

    Ok(())
}

fn process_ls_output_line(
    root: &mut DirectoryNode,
    current_path: &Vec<String>,
    entry_info: &str,
    entry_name: &str)
    -> Result<(), Error>
{
    let current_dir = root.get_dir(current_path).unwrap();

    if entry_info == "dir" {
        if !current_dir.directories.contains_key(entry_name) {
            let d = DirectoryNode::new(String::from(entry_name));
            current_dir.directories.insert(String::from(entry_name), d);
        }
    }
    else {
        match entry_info.parse::<u32>() {
            Err(_) => {
                return Err(Error::General(format!("Invalid output entyr for ls: {}", entry_info)));
            },
            Ok(size) => {
                let f = FileNode::new(String::from(entry_name), size);
                current_dir.files.insert(String::from(entry_name), f);
            }
        }

    }

    Ok(())
}

fn parse_lines(lines: &Vec<String>) -> Result<DirectoryNode, Error> {
    let mut root = DirectoryNode::new(String::from(""));
    let mut current_path: Vec<String> = Vec::new();
    let mut last_command: String = String::from("");

    for line in lines {
        let line_parts = line.split(" ").collect::<Vec<&str>>();
        
        // Process command.
        if line_parts.len() >= 2 && line_parts[0] == "$" {
            last_command = String::from(line_parts[1]);

            if line_parts.len() >= 3 && line_parts[1] == "cd" {
                process_cd(&mut root, &mut current_path, &line_parts[2])?;
            }
            else if line_parts[1] == "ls" {
                // Nothing to do, just need to know in ls command.
            }
            else {
                return Err(Error::General(format!("Unknown command: {}", line_parts[1])));
            }
        }
        // Process out put lines.
        else {
            if last_command == "ls" {
                process_ls_output_line(
                    &mut root,
                    &current_path,
                    line_parts[0],
                    line_parts[1]
                )?;
            }
            else {
                return Err(Error::General(format!("Can't process output from command: {}", last_command)));
            }
        }
    }

    Ok(root)
}

// TODO: Seems like there should be an easier way to pass closures which capture state.
fn process_directories<T>(
    root: &DirectoryNode,
    state: &mut T,
    callback: fn(state: &mut T, path: &Vec<String>, total_size: u32) -> ())
{
    fn _process<T>(
        path: &mut Vec<String>,
        node: &DirectoryNode,
        state: &mut T,
        callback: fn(state: &mut T, path: &Vec<String>, total_size: u32) -> ())
        -> u32
    {
        let mut total_size: u32 = 0;
    
        for file_node in node.files.values() {
            total_size += file_node.size; 
        }
    
        for dir_node in node.directories.values() {
            path.push(String::from(&dir_node.name));
            total_size += _process(path, dir_node, state, callback);
            path.pop();
        }
    
        callback(state, path, total_size);
    
        total_size
    }

    let mut path: Vec<String> = Vec::new();
    _process(&mut path, root, state, callback);
}

fn calculate_part1_result(root: &DirectoryNode) -> u32 {
    let mut result: u32 = 0;

    process_directories::<u32>(root, &mut result, |state, _path, total_size| {
        if total_size <= 100000 {
            *state = *state + total_size;
        }
    });

    result
}

fn get_root_size(root: &DirectoryNode) -> u32 {
    let mut result: u32 = 0;

    process_directories::<u32>(root, &mut result, |state, path, total_size| {
        if path.len() == 0 {
            *state = total_size;
        }
    });

    result
}

pub fn part1(input_path: &str) -> Result<String, Error> {
    let lines = read_non_empty_lines(input_path)?;
    let root = parse_lines(&lines)?;
    let result = calculate_part1_result(&root);

    Ok(result.to_string())
}

struct Part2State {
    need_to_free: u32,
    dir_to_delete: Option<String>,
    dir_to_delete_size: Option<u32>,
}

pub fn part2(input_path: &str) -> Result<String, Error> {
    let lines = read_non_empty_lines(input_path)?;
    let root = parse_lines(&lines)?;
    let root_size = get_root_size(&root);

    let total_capacity: u32 = 70000000;
    let free_space_needed: u32 = 30000000;
    let max_usable_space: u32 = total_capacity - free_space_needed;

    if root_size <= max_usable_space {
        return Ok(String::from("No deletion needed."))
    }

    let mut state = Part2State {
        need_to_free: root_size - max_usable_space,
        dir_to_delete: None,
        dir_to_delete_size: None,
    };

    process_directories::<Part2State>(&root, &mut state, |state, path, total_size| {
        if total_size >= state.need_to_free {
            if state.dir_to_delete.is_none() || state.dir_to_delete_size.unwrap() > total_size {
                state.dir_to_delete = Some(path.join("/"));
                state.dir_to_delete_size = Some(total_size);
            }
        }
    });

    Ok(format!("{}", state.dir_to_delete_size.unwrap()))
}