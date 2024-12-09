use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn parse(file: String) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start = (0, 0);
    for (r, line) in file.lines().enumerate() {
        let mut cur_line = Vec::new();
        for (c, ch) in line.chars().enumerate() {
            cur_line.push(ch);
            if ch == '^' {
                start = (r, c);
            }
        }
        grid.push(cur_line);
    }

    (grid, start)
}

fn traverse_until_bounds(grid: Vec<Vec<char>>, mut current_pos: (usize, usize)) -> usize {
    let direction: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut cur_dir = 0;

    visited.insert(current_pos);

    loop {
        let new_x = current_pos.0 as isize + direction[cur_dir].0;
        let new_y = current_pos.1 as isize + direction[cur_dir].1;

        if new_x < 0 || new_x as usize >= grid.len() || new_y < 0 || new_y as usize >= grid[0].len()
        {
            break;
        }

        let new_pos = (new_x as usize, new_y as usize);

        if grid[new_pos.0][new_pos.1] == '#' {
            cur_dir = (cur_dir + 1) % direction.len();
        } else {
            current_pos = new_pos;
            visited.insert(current_pos);
        }
    }

    visited.len()
}

fn traverse_until_loop(grid: Vec<Vec<char>>, mut current_pos: (usize, usize)) -> bool {
    let direction: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut visited: HashMap<(usize, usize), (isize, isize)> = HashMap::new();
    let mut cur_dir = 0;

    visited.insert(current_pos, direction[cur_dir]);

    loop {
        let new_x = current_pos.0 as isize + direction[cur_dir].0;
        let new_y = current_pos.1 as isize + direction[cur_dir].1;

        if new_x < 0 || new_x as usize >= grid.len() || new_y < 0 || new_y as usize >= grid[0].len()
        {
            break;
        }

        let new_pos = (new_x as usize, new_y as usize);

        if let Some(&stored_dir) = visited.get(&new_pos) {
            if stored_dir == direction[cur_dir] {
                println!("Loop detected");
                return true;
            }
        }

        if grid[new_pos.0][new_pos.1] == '#' {
            cur_dir = (cur_dir + 1) % direction.len();
        } else {
            current_pos = new_pos;
            visited.insert(current_pos, direction[cur_dir]);
        }
    }

    false
}

fn part1(grid: Vec<Vec<char>>, mut current_pos: (usize, usize)) -> usize {
    traverse_until_bounds(grid, current_pos)
}

fn part2(mut grid: Vec<Vec<char>>, mut current_pos: (usize, usize)) -> usize {
    let mut result = 0;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let mut cur_grid = grid.clone();
            if cur_grid[r][c] == '#' {
                continue;
            }
            if cur_grid[r][c] == '.' {
                cur_grid[r][c] = '#';

                if traverse_until_loop(cur_grid.clone(), current_pos) {
                    result += 1;
                }
            }
        }
    }

    result
}

fn main() {
    let file = fs::read_to_string("./src/input.txt").unwrap();

    let (grid, initial_pos) = parse(file);

    let result1 = part1(grid.clone(), initial_pos);
    let result2 = part2(grid.clone(), initial_pos);

    println!("{:?}", result1);
    println!("{:?}", result2);
}
