use std::fs;

fn parse(file: String) -> Vec<Vec<char>> {
    let mut all_lines: Vec<Vec<char>> = Vec::new();

    for line in file.lines() {
        let cur_line = line.chars().collect();
        all_lines.push(cur_line);
    }

    all_lines
}

fn is_xmas(r: usize, c: usize, grid: &Vec<Vec<char>>) -> usize {
    if grid[r][c] != 'X' {
        return 0;
    }

    let mut count = 0;

    for dr in [-1i32, 0, 1].iter() {
        for dc in [-1i32, 0, 1].iter() {
            let dr = *dr;
            let dc = *dc;
            if dr == 0 && dc == 0 {
                continue;
            }

            let r = r as i32;
            let c = c as i32;

            if !(0 <= r as i32 + dr * 3
                && 0 <= c as i32 + dc * 3
                && r as i32 + dr * 3 < grid[0].len() as i32
                && c as i32 + dc * 3 < grid.len() as i32)
            {
                continue;
            }
            if grid[(r + dr) as usize][(c + dc) as usize] == 'M'
                && grid[(r + dr * 2) as usize][(c + dc * 2) as usize] == 'A'
                && grid[(r + dr * 3) as usize][(c + dc * 3) as usize] == 'S'
            {
                println!("Starting X from: {r:?}  {c:?}");
                println!("Going to: {dr}  {dc}");
                count += 1;
            }
        }
    }
    count
}

fn is_x_mas(r: usize, c: usize, grid: Vec<Vec<char>>) -> usize {
    let mut result = 0;

    if grid[r][c] != 'A' {
        return 0;
    }

    let corners: [char; 4] = [
        grid[r - 1][c - 1],
        grid[r - 1][c + 1],
        grid[r + 1][c + 1],
        grid[r + 1][c - 1],
    ];

    if matches!(
        corners,
        ['M', 'M', 'S', 'S'] | ['S', 'S', 'M', 'M'] | ['M', 'S', 'S', 'M'] | ['S', 'M', 'M', 'S']
    ) {
        result += 1;
    }

    println!("{corners:?}");

    result
}

fn part1(grid: Vec<Vec<char>>) {
    let mut result = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, col) in grid.iter().enumerate() {
            result += is_xmas(r, c, &grid);
        }
    }

    println!("{:?}", result);
}

fn part2(grid: Vec<Vec<char>>) {
    let mut result = 0;
    for r in 1..grid.len() - 1 {
        for c in 1..grid.len() - 1 {
            result += is_x_mas(r, c, grid.clone());
        }
    }

    println!("{:?}", result);
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();

    let parsed_lines = parse(input);
    part1(parsed_lines.clone());
    part2(parsed_lines);
}
