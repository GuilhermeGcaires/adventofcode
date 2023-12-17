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

    let mut current_nodes: Vec<String> = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect();

    while !current_nodes.iter().all(|node| node.ends_with('Z')) {
        let mut result = Vec::new();

        for current_node in &current_nodes {
            for instruction in instructions.chars() {
                count += 1;
                let next_node = if instruction == 'L' {
                    map.get(current_node).unwrap()[0].clone()
                } else if instruction == 'R' {
                    map.get(current_node).unwrap()[1].clone()
                } else {
                    current_node.clone()
                };
                result.push(next_node);
            }
        }

        current_nodes = result;
    }

    println!("{:?}", count);
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
