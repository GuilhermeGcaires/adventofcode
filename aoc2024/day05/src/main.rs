use std::{cmp::Ordering, collections::HashSet, fs};

fn parse(file: String) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
    let (rules_section, updates_section) = file.split_once("\n\n").unwrap();

    let rules: HashSet<(u32, u32)> = rules_section
        .lines()
        .map(|line| {
            let parts: Vec<u32> = line.split('|').map(|num| num.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    let updates: Vec<Vec<u32>> = updates_section
        .lines()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    println!("{:?}  {:?}", rules_section, updates_section);

    (rules, updates)
}

fn is_valid_update(update: &Vec<u32>, rules: &HashSet<(u32, u32)>) -> bool {
    for &(x, y) in rules.iter() {
        let x_idx = update.iter().position(|&p| p == x);
        let y_idx = update.iter().position(|&p| p == y);

        if let (Some(x_pos), Some(y_pos)) = (x_idx, y_idx) {
            if x_pos > y_pos {
                return false;
            }
        }
    }
    true
}

fn part1(rules: HashSet<(u32, u32)>, updates: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;

    for update in updates.iter() {
        if is_valid_update(update, &rules) {
            let middle_index = update.len() / 2;
            sum += update[middle_index];
        }
    }

    sum
}

fn part2(rules: HashSet<(u32, u32)>, updates: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;

    for update in updates.iter() {
        if is_valid_update(update, &rules) {
            continue;
        }
        let mut sorted_update = update.clone();

        sorted_update.sort_by(|&a, &b| {
            if a == b {
                Ordering::Equal
            } else if rules.contains(&(a, b)) {
                Ordering::Less
            } else if rules.contains(&(b, a)) {
                Ordering::Greater
            } else {
                unreachable!()
            }
        });

        sum += sorted_update[sorted_update.len() / 2];

        println!("Sorted update: {:?}", sorted_update);
    }

    sum
}

fn main() {
    let file = fs::read_to_string("./src/input.txt").unwrap();

    let (rules, updates) = parse(file);

    println!("{:?}", part1(rules.clone(), updates.clone()));
    println!("{:?}", part2(rules, updates));
}
