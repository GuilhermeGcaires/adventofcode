use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
enum Pipe {
    NorthToSouth, // |
    EastToWest,   // -
    NorthToEast,  // L
    NorthToWest,  // J
    SouthToWest,  // 7
    SouthToEast,  // F
    Soil,         // .
    Start,
}

struct Puzzle {
    labyrinth: Vec<Vec<Pipe>>,
    starting_pos: (usize, usize),
}

fn part1(puzzle: Puzzle) {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue = VecDeque::new();

    //visited.insert(puzzle.starting_pos);
    queue.push_back((puzzle.starting_pos));

    while let Some(coordinates) = queue.pop_front() {
        let (r, c) = coordinates;
        let cur_pipe = &puzzle.labyrinth[r][c];

        if r > 0
            && matches!(
                cur_pipe,
                Pipe::NorthToSouth | Pipe::Start | Pipe::NorthToWest | Pipe::NorthToEast
            )
            && matches!(
                puzzle.labyrinth[r - 1][c],
                Pipe::NorthToSouth | Pipe::SouthToWest | Pipe::SouthToEast
            )
            && !visited.contains(&(r - 1, c))
        {
            queue.push_back((r - 1, c));
            visited.insert((r - 1, c));
        }

        if r < puzzle.labyrinth.len() - 1
            && matches!(
                cur_pipe,
                Pipe::Start | Pipe::NorthToSouth | Pipe::SouthToWest | Pipe::SouthToEast
            )
            && matches!(
                puzzle.labyrinth[r + 1][c],
                Pipe::NorthToSouth | Pipe::NorthToWest | Pipe::NorthToEast
            )
            && !visited.contains(&(r + 1, c))
        {
            queue.push_back((r + 1, c));
            visited.insert((r + 1, c));
        }

        if c > 0
            && matches!(
                cur_pipe,
                Pipe::Start | Pipe::EastToWest | Pipe::NorthToWest | Pipe::SouthToWest
            )
            && matches!(
                puzzle.labyrinth[r][c - 1],
                Pipe::EastToWest | Pipe::SouthToEast | Pipe::NorthToEast
            )
            && !visited.contains(&(r, c - 1))
        {
            queue.push_back((r, c - 1));
            visited.insert((r, c - 1));
        }

        if c < puzzle.labyrinth[0].len() - 1
            && matches!(
                cur_pipe,
                Pipe::Start | Pipe::EastToWest | Pipe::NorthToEast | Pipe::SouthToEast
            )
            && matches!(
                puzzle.labyrinth[r][c + 1],
                Pipe::EastToWest | Pipe::NorthToWest | Pipe::SouthToWest
            )
            && !visited.contains(&(r, c + 1))
        {
            queue.push_back((r, c + 1));
            visited.insert((r, c + 1));
        }
    }
    println!("Result = {:?}", visited.len().div_ceil(2));
}

fn parse(input: String) -> Puzzle {
    let mut starting_pos: (usize, usize) = (0, 0);
    let labyrinth = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, ch)| match ch {
                    '|' => Pipe::NorthToSouth,
                    '-' => Pipe::EastToWest,
                    'L' => Pipe::NorthToEast,
                    'J' => Pipe::NorthToWest,
                    '7' => Pipe::SouthToWest,
                    'F' => Pipe::SouthToEast,
                    '.' => Pipe::Soil,
                    'S' => {
                        starting_pos = (row, col);
                        Pipe::Start
                    }
                    _ => Pipe::Soil,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Puzzle {
        labyrinth,
        starting_pos,
    }
}

fn main() {
    let input = fs::read_to_string("./input/input.txt").expect("Could not read file");

    let puzzle = parse(input);
    part1(puzzle);
}
