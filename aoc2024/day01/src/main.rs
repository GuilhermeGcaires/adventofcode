use std::{collections::HashMap, fs};

fn part1(input: String) {
    let mut left_list: Vec<i64> = Vec::new();
    let mut right_list: Vec<i64> = Vec::new();

    let _ = input
        .lines()
        .map(|line| {
            if let Some((l, r)) = line.split_once("   ") {
                let l = l.parse::<i64>().unwrap();
                let r = r.parse::<i64>().unwrap();

                left_list.push(l);
                right_list.push(r);
            }
        })
        .collect::<Vec<_>>();

    left_list.sort();
    right_list.sort();

    let mut result: i64 = 0;

    for i in 0..left_list.len() {
        let diff = (left_list[i] - right_list[i]).abs();
        result += diff;
    }

    println!("{}", result);
}

fn part2(input: String) {
    let mut left_list: Vec<i64> = Vec::new();
    let mut right_list: HashMap<i64, i64> = HashMap::new();

    let _ = input
        .lines()
        .map(|line| {
            if let Some((l, r)) = line.split_once("   ") {
                let l = l.parse::<i64>().unwrap();
                let r = r.parse::<i64>().unwrap();

                left_list.push(l);
                *right_list.entry(r).or_insert(0) += 1;
            }
        })
        .collect::<Vec<_>>();

    left_list.sort();

    let mut result: i64 = 0;

    for i in 0..left_list.len() {
        let left_num = left_list[i];

        let occurrences = right_list.entry(left_num).or_default();

        result += left_num * *occurrences;
    }
    println!("Part2: {}", result);
}

fn main() {
    let file = fs::read_to_string("./src/input.txt").unwrap();

    part1(file.clone());
    part2(file);
}
