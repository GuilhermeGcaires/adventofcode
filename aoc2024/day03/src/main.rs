use std::fs;

use regex::Regex;

fn part1(input: String) {
    let mut result = 0;
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    for capture in re.captures_iter(&input) {
        let num1: i32 = capture[1].parse().unwrap();
        let num2: i32 = capture[2].parse().unwrap();
        result += num1 * num2;
    }
    println!("{result:?}");
}

fn part2(input: String) {
    let mut result = 0;
    let mut enabled = true;
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();
    for capture in re.captures_iter(&input) {
        if let Some(matched) = capture.get(0) {
            let matched_str = matched.as_str();
            if matched_str == "do()" {
                enabled = true;
            } else if matched_str == "don't()" {
                enabled = false;
            } else if enabled == true {
                if let (Some(num1), Some(num2)) = (capture.get(1), capture.get(2)) {
                    let num1: i32 = num1.as_str().parse().unwrap();
                    let num2: i32 = num2.as_str().parse().unwrap();
                    result += num1 * num2;
                }
            }
            println!("Matched: {:?}", matched_str);
        }
    }
    println!("{result:?}");
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();

    part1(input.clone());
    part2(input);
}
