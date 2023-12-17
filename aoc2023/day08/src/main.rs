use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./input/input.txt").expect("Could not read file");

    let (map, instructions, _first_line) = parse(&input);
    process_part1(&map, &instructions);
    process_part2(&map, &instructions);
}

fn process_part1(map: &HashMap<String, Vec<String>>, instructions: &String) {
    let mut count = 0;
    let mut current = "AAA".to_string();
    while current != "ZZZ" {
        for instruction in instructions.chars() {
            count += 1;
            if instruction == 'L' {
                current = map.get(&current).unwrap()[0].clone();
            } else if instruction == 'R' {
                current = map.get(&current).unwrap()[1].clone();
            }
        }
    }
    println!("{:?}", count);
}

fn process_part2(map: &HashMap<String, Vec<String>>, instructions: &String) {
    let mut count = 0;
    let dirs: Vec<usize> = instructions
        .chars()
        .map(|ch| if ch == 'L' { 0 } else { 1 })
        .collect();
    let queue = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<_>>();

    let mut ans = Vec::new();
    for entry in queue {
        let mut current = entry.clone();
        let mut count = 0;

        while !current.ends_with('Z') {
            current = get(&map, &current, &dirs, count);
            count += 1;
        }

        ans.push(count)
    }

    println!("{:?}", lcm_of(&ans));
}

fn parse(input: &str) -> (HashMap<String, Vec<String>>, String, (String, Vec<String>)) {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let instructions = input.lines().nth(0).unwrap().to_string();
    let first_line = parse_line(input.lines().nth(2).unwrap());
    for line in input.lines().skip(2) {
        let (root, branches) = parse_line(line);

        map.insert(root.to_string(), branches);
    }
    (map, instructions, first_line)
}

fn parse_line(line: &str) -> (String, Vec<String>) {
    let (root, rest) = line.split_once(" = ").unwrap();
    let (_, rest) = rest.split_once("(").unwrap();
    let (rest, _) = rest.split_once(")").unwrap();
    let (left, right) = rest.split_once(", ").unwrap();
    let branches = vec![left.to_string(), right.to_string()];

    (root.to_string(), branches)
}

fn get(
    map: &HashMap<String, Vec<String>>,
    current: &String,
    dirs: &Vec<usize>,
    count: usize,
) -> String {
    let instr_len = dirs.len();
    map.get(current).unwrap()[dirs[count % instr_len]].clone()
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn _gcd_of(list: &[usize]) -> usize {
    let mut iter = list.iter();
    let first = *iter.next().unwrap();
    let second = *iter.next().unwrap();

    let mut ans = gcd(first, second);
    for next in iter {
        ans = gcd(ans, *next)
    }

    ans
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

fn lcm_of(list: &[usize]) -> usize {
    let mut iter = list.iter();
    let first = *iter.next().unwrap();
    let second = *iter.next().unwrap();

    let mut ans = lcm(first, second);
    for next in iter {
        ans = lcm(ans, *next)
    }

    ans
}
