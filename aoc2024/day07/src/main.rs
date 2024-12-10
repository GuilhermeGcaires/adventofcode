use std::fs;

#[derive(Debug, Clone)]
struct Calibration {
    desired: u64,
    nums: Vec<u64>,
}

fn parse(input: String) -> Vec<Calibration> {
    let mut parsed_input: Vec<Calibration> = Vec::new();
    for line in input.lines() {
        if let Some((left, right)) = line.split_once(':') {
            let left = left.parse::<u64>().unwrap();

            let nums_vector: Vec<u64> = right
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();

            let calibration = Calibration {
                desired: left,
                nums: nums_vector,
            };
            parsed_input.push(calibration);
        }
    }
    parsed_input
}

fn recurse(i: usize, cur_sum: u64, calibration: Calibration) -> bool {
    if i == calibration.nums.len() {
        return cur_sum == calibration.desired;
    }

    if recurse(i + 1, cur_sum + calibration.nums[i], calibration.clone()) {
        return true;
    }

    if recurse(i + 1, cur_sum * calibration.nums[i], calibration) {
        return true;
    }

    false
}

fn recurse_2(i: usize, cur_sum: u64, calibration: Calibration) -> bool {
    if i == calibration.nums.len() {
        return cur_sum == calibration.desired;
    }

    if recurse_2(i + 1, cur_sum + calibration.nums[i], calibration.clone()) {
        return true;
    }

    if recurse_2(i + 1, cur_sum * calibration.nums[i], calibration.clone()) {
        return true;
    }

    let current_num = calibration.nums[i];

    let concatenated = format!("{}{}", cur_sum, current_num)
        .parse::<u64>()
        .unwrap();

    if recurse_2(i + 1, concatenated, calibration) {
        return true;
    }

    false
}

fn part1(calibrations: Vec<Calibration>) -> u64 {
    let mut result = 0;
    for calibration in calibrations {
        if recurse(0, 0, calibration.clone()) {
            result += calibration.desired;
        }
    }
    result
}

fn part2(calibrations: Vec<Calibration>) -> u64 {
    let mut result = 0;
    for calibration in calibrations {
        if recurse_2(0, 0, calibration.clone()) {
            result += calibration.desired;
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let parsed_input = parse(input);

    let result1 = part1(parsed_input.clone());
    println!("Result 1: {result1:?}");

    let result2 = part2(parsed_input);
    println!("Result 2: {result2:?}");
}
