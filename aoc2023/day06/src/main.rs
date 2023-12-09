use std::fs;
use std::iter::zip;

fn main() {
    let input = fs::read_to_string("./input/input.txt").expect("Could not read file");

    let (times, distances) = parse(&input);
    process_part1(&times, &distances);

    let concatenated_times: String = times.iter().map(|&num| num.to_string()).collect();
    let time_part2 = concatenated_times.parse::<i64>().unwrap();
    let concatenated_distances: String = distances.iter().map(|&num| num.to_string()).collect();
    let distance_part2 = concatenated_distances.parse::<i64>().unwrap();

    process_part2(time_part2, distance_part2);
}

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    let times: Vec<i64> = input
        .lines()
        .nth(0)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();

    let distances: Vec<i64> = input
        .lines()
        .nth(1)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();

    println!("{:?}", times);
    println!("{:?}", distances);
    (times, distances)
}

fn process_part1(times: &Vec<i64>, distances: &Vec<i64>) {
    let mut all_nums: Vec<i64> = Vec::new();
    for (time, distance) in zip(times, distances) {
        let mut i = 1;
        let mut record_beaten = 0;
        while i < *time {
            let remaining_time = time - i;
            if remaining_time * i > *distance {
                record_beaten += 1;
            }
            i += 1;
        }
        all_nums.push(record_beaten);
    }
    let result_product: i64 = all_nums.iter().product();
    println!("Part 1: {:?}", result_product);
}

fn process_part2(time: i64, distance: i64) {
    let mut i = 0;
    let mut count = 0;

    while i < time {
        let remaining_time = time - i;
        if remaining_time * i > distance {
            count += 1;
        }
        i += 1;
    }
    println!("Part 2: {:?}", count);
}
