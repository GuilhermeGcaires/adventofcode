use std::{
    char,
    collections::{HashMap, HashSet},
    fs,
};

fn parse(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(grid: Vec<Vec<char>>) -> usize {
    let mut antenas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for r in 0..grid.len() as isize {
        for c in 0..grid[0].len() as isize {
            if grid[r as usize][c as usize] != '.' {
                let ch = grid[r as usize][c as usize];
                antenas.entry(ch).or_insert_with(Vec::new).push((r, c));
            }
        }
    }

    for array in antenas.values() {
        for i in 0..array.len() {
            for j in i + 1..array.len() {
                let (r1, c1) = array[i];
                let (r2, c2) = array[j];

                antinodes.insert((2 * r1 - r2, 2 * c1 - c2));
                antinodes.insert((2 * r2 - r1, 2 * c2 - c1));
            }
        }
    }

    println!("Row length: {rows:?}, Col length: {cols:?}");
    antinodes
        .iter()
        .filter(|node| 0 <= node.0 && node.0 < rows && 0 <= node.1 && node.1 < cols)
        .count()
}

fn part2(grid: Vec<Vec<char>>) -> usize {
    let mut antenas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for r in 0..grid.len() as isize {
        for c in 0..grid[0].len() as isize {
            if grid[r as usize][c as usize] != '.' {
                let ch = grid[r as usize][c as usize];
                antenas.entry(ch).or_insert_with(Vec::new).push((r, c));
            }
        }
    }

    for array in antenas.values() {
        for i in 0..array.len() {
            for j in 0..array.len() {
                if i == j {
                    continue;
                }
                let (r1, c1) = array[i];
                let (r2, c2) = array[j];

                let dr = r2 - r1;
                let dc = c2 - c1;
                let mut r = r1;
                let mut c = c1;

                while 0 <= r && r < rows && 0 <= c && c < cols {
                    antinodes.insert((r, c));
                    r += dr;
                    c += dc;
                }
            }
        }
    }

    println!("Row length: {rows:?}, Col length: {cols:?}");
    antinodes
        .iter()
        .filter(|node| 0 <= node.0 && node.0 < rows && 0 <= node.1 && node.1 < cols)
        .count()
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let grid = parse(input);

    let result1 = part1(grid.clone());
    let result2 = part2(grid);

    println!("Part 1: {}", result1);
    println!("Part 2: {:?}", result2);
}
