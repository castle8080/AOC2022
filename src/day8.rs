use crate::common::{Error, read_non_empty_lines};
use std::collections::HashSet;

fn parse_input(input_path: &str) -> Result<Vec<Vec<u8>>, Error> {
    let mut trees: Vec<Vec<u8>> = Vec::new();

    for line in read_non_empty_lines(input_path)? {
        let mut row: Vec<u8> = Vec::new();
        for c in line.chars() {
            if c < '0' || c > '9' {
                return Err(Error::General(format!("Invalid character: {}", c)));
            }
            else {
                let h = ((c as u32) - ('0' as u32)) as u8;
                row.push(h);
            }
        }
        if trees.len() > 0 && trees[0].len() != row.len() {
            return Err(Error::General(format!("Row has a mismatched length.")));
        }
        trees.push(row);
    }

    Ok(trees)
}

fn is_in_bounds(pos: (i16, i16), width: i16, height: i16) -> bool {
    pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height
}

fn get_bounds(trees: &Vec<Vec<u8>>) -> (i16, i16) {
    let height = trees.len() as i16;
    let width = if height > 0 { trees[0].len() as i16 } else { 0 };

    (width, height)
}

fn pos_add_vector(pos: (i16, i16), move_vec: (i16, i16)) -> (i16, i16) {
    (pos.0 + move_vec.0, pos.1 + move_vec.1)
}

fn mark_visible(
    visible: &mut HashSet<(i16, i16)>,
    trees: &Vec<Vec<u8>>,
    pos: (i16, i16),
    move_vec: (i16, i16))
{
    let mut current_pos = pos;
    let (width, height) = get_bounds(trees);
    let mut max_height: i16 = -1;

    while is_in_bounds(current_pos, width, height) {
        let t_height = trees[current_pos.1 as usize][current_pos.0 as usize] as i16;
        if t_height > max_height {
            visible.insert(current_pos);
            max_height = t_height;
        }
        current_pos = pos_add_vector(current_pos, move_vec);
    }
}

fn get_visible_trees(trees: &Vec<Vec<u8>>) -> HashSet<(i16, i16)> {
    let mut visible: HashSet<(i16, i16)> = HashSet::new();
    let (width, height) = get_bounds(trees);

    for x in 0..width {
        mark_visible(&mut visible, trees, (x as i16, 0), (0, 1));
        mark_visible(&mut visible, trees, (x as i16, (height - 1) as i16), (0, -1));
    }

    for y in 0..height {
        mark_visible(&mut visible, trees, (0, y as i16), (1, 0));
        mark_visible(&mut visible, trees, ((width - 1) as i16, y as i16), (-1, 0));
    }

    return visible;
}

pub fn part1(input_path: &str) -> Result<String, Error> {
    let trees = parse_input(input_path)?;
    let visible = get_visible_trees(&trees);
    let result = visible.len();

    Ok(result.to_string())
}

fn get_visibility_count_direction(trees: &Vec<Vec<u8>>, pos: (i16, i16), move_vec: (i16, i16)) -> i32 {
    let (width, height) = get_bounds(trees);
    let tree_height = trees[pos.1 as usize][pos.0 as usize];
    let mut visibility_count: i32 = 0;

    let mut current_pos = pos;
    loop {
        current_pos = pos_add_vector(current_pos, move_vec);
        if !is_in_bounds(current_pos, width, height) {
            break;
        }
        visibility_count += 1;
        
        let current_tree_height = trees[current_pos.1 as usize][current_pos.0 as usize];
        if current_tree_height >= tree_height {
            break;
        }
    }

    visibility_count
}

fn get_scenic_score(trees: &Vec<Vec<u8>>, pos: (i16, i16)) -> i32 {
    get_visibility_count_direction(trees, pos, (0, 1)) * 
    get_visibility_count_direction(trees, pos, (0, -1)) * 
    get_visibility_count_direction(trees, pos, (1, 0)) *
    get_visibility_count_direction(trees, pos, (-1, 0))
}

fn get_scenic_scores(trees: &Vec<Vec<u8>>) -> Vec<(i16, i16, i32)> {
    let (width, height) = get_bounds(trees);
    let mut scenic_scores: Vec<(i16, i16, i32)> = Vec::new(); 

    for x in 0..width {
        for y in 0..height {
            let pos = (x as i16, y as i16);
            let scenic_score = get_scenic_score(trees, pos);
            scenic_scores.push((x as i16, y as i16, scenic_score));
        }
    }

    scenic_scores
}

pub fn part2(input_path: &str) -> Result<String, Error> {
    let trees = parse_input(input_path)?;
    let scenic_scores = get_scenic_scores(&trees);

    let best = scenic_scores
        .iter()
        .max_by_key(|(_x, _y, scenic_score)| scenic_score)
        .unwrap();

    Ok(best.2.to_string())
}