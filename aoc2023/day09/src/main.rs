use std::fs;

use itertools::Itertools;

fn part1(input: String) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut values: Vec<i64> = line
                .split_ascii_whitespace()
                .map(|val| val.parse().unwrap())
                .collect();
            let mut prediction = 0;

            while !values.iter().all(|val| *val == 0) {
                prediction += *values.last().unwrap();
                values = values
                    .into_iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect();
            }
            prediction
        })
        .sum()
}

fn part2(input: String) -> i64 {
    let sum: i64 = input
        .lines()
        .map(|line| {
            let mut values: Vec<_> = line
                .split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            let mut prediction = 0;
            let mut sign = -1;
            while !values.iter().all(|v| *v == 0) {
                prediction = *values.first().unwrap() - prediction;
                sign *= -1;
                values = values
                    .into_iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect();
            }
            sign * prediction
        })
        .sum();

    sum
}

fn main() {
    let input = fs::read_to_string("./input/input.txt").expect("Could not read file");

    let result_part1 = part1(input.clone());
    println!("{:?}", result_part1);

    let result_part2 = part2(input);
    println!("{:?}", result_part2);
}
