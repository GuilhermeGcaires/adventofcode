use std::fs;

fn main() {
    let input = fs::read_to_string("./input/input.txt").expect("Could not read the file.");

    part1(&input);
    println!("{:?}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut left = 0;
        let mut right = 0;
        for ch in line.chars() {
            if ch.is_numeric() {
                if left == 0 && right == 0 {
                    left = ch.to_digit(10).unwrap() as u64;
                    right = ch.to_digit(10).unwrap() as u64;
                } else {
                    right = ch.to_digit(10).unwrap() as u64;
                }
            }
        }
        sum += left * 10 + right;
    }

    println!("{:?}", sum);

    sum
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.to_string()
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum()
}
