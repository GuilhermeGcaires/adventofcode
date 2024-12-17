use std::{collections::HashSet, fs::read_to_string};

const DIRECTIONS: [(i32, i32); 4] = [
    (0, 1),  // Right
    (1, 0),  // Down
    (0, -1), // Left
    (-1, 0), // Up
];

fn parse(input: String) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn dfs(
    grid: &Vec<Vec<u8>>,
    r: usize,
    c: usize,
    visited: &mut HashSet<(usize, usize)>,
    step: u8,
) -> usize {
    if step > 9 || grid[r][c] != step || visited.contains(&(r, c)) {
        return 0;
    }

    visited.insert((r, c));

    if step == 9 {
        return 1;
    }

    let mut count = 0;

    for (dr, dc) in DIRECTIONS {
        let new_r = r as isize + dr as isize;
        let new_c = c as isize + dc as isize;

        if new_r >= 0
            && new_c >= 0
            && (new_r as usize) < grid.len()
            && (new_c as usize) < grid[0].len()
        {
            count += dfs(grid, new_r as usize, new_c as usize, visited, step + 1);
        }
    }

    count
}

fn part1(grid: Vec<Vec<u8>>) -> usize {
    let mut result = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 0 {
                let mut visited = HashSet::new();
                result += dfs(&grid, r, c, &mut visited, 0);
            }
        }
    }

    result
}

fn dfs_count_trails(
    grid: &Vec<Vec<u8>>,
    r: usize,
    c: usize,
    step: u8,
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    if step > 9 || grid[r][c] != step || visited.contains(&(r, c)) {
        return 0;
    }

    if step == 9 {
        return 1;
    }

    visited.insert((r, c));
    let mut trail_count = 0;

    for (dr, dc) in DIRECTIONS {
        let new_r = r as isize + dr as isize;
        let new_c = c as isize + dc as isize;

        if new_r >= 0
            && new_c >= 0
            && (new_r as usize) < grid.len()
            && (new_c as usize) < grid[0].len()
        {
            trail_count +=
                dfs_count_trails(grid, new_r as usize, new_c as usize, step + 1, visited);
        }
    }

    visited.remove(&(r, c));
    trail_count
}

fn part2(grid: Vec<Vec<u8>>) -> usize {
    let mut total_rating = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 0 {
                let mut visited = HashSet::new();
                total_rating += dfs_count_trails(&grid, r, c, 0, &mut visited);
            }
        }
    }

    total_rating
}
fn main() {
    let input = read_to_string("./src/input.txt").unwrap();
    let parsed = parse(input);

    let result = part1(parsed.clone());
    println!("Result: {}", result);

    let result2 = part2(parsed);
    println!("Result: {}", result2);
}
