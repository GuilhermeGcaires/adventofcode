use std::fs;

#[derive(Debug)]
pub struct Game {
    number: u32,
    set: Vec<Turn>,
}

impl Game {
    fn new(number: u32, set: Vec<Turn>) -> Game {
        Self { number, set }
    }
}

#[derive(Default, Debug)]
pub struct Turn {
    red: usize,
    green: usize,
    blue: usize,
}

impl Turn {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn main() {
    let input = fs::read_to_string("./input/input.txt").expect("unable to read file");
    let game_structure = parse(&input);

    process_part1(&game_structure);
    process_part2(&game_structure);
}

fn parse(input: &str) -> Vec<Game> {
    let mut all_games: Vec<Game> = Vec::new();
    for line in input.lines() {
        let (game, turns) = line.split_once(": ").unwrap();
        let game = game.split(" ").nth(1).unwrap().parse().unwrap();
        let turns = turns.split("; ").collect::<Vec<_>>();
        let mut all_turns = Vec::new();

        for t in turns {
            let cubes = t.split(", ").collect::<Vec<_>>();
            let mut turn = Turn::default();
            for c in cubes {
                let (amount, color) = c.split_once(" ").unwrap();
                let amount: usize = amount.parse().unwrap();
                match color {
                    "red" => turn.red = amount,
                    "blue" => turn.blue = amount,
                    "green" => turn.green = amount,
                    _ => panic!("unexpected color"),
                }
            }
            all_turns.push(turn);
        }
        let cur_game = Game::new(game, all_turns);
        all_games.push(cur_game);
    }

    all_games
}

fn process_part1(games: &Vec<Game>) {
    let mut valid_games = 0;

    'next_game: for game in games {
        for turn in &game.set {
            if !turn.is_valid() {
                continue 'next_game;
            }
        }
        valid_games += game.number;
    }
    println!("{:?}", valid_games);
}

fn process_part2(games: &Vec<Game>) {
    let mut total_sum = 0;

    for game in games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for turn in &game.set {
            red = red.max(turn.red);
            green = green.max(turn.green);
            blue = blue.max(turn.blue);
        }
        total_sum += red * green * blue;
    }
    println!("{:?}", total_sum);
}
