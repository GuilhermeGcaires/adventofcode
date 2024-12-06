use std::fs;

fn part1(input: String) {
    let mut result = 0;
    for line in input.lines() {
        let cur_line: Vec<i64> = line
            .split(" ")
            .map(|num| num.parse::<i64>().unwrap())
            .collect();

        let mut only_increasing = true;
        let mut only_decreasing = true;
        let mut ok = true;

        for i in 1..cur_line.len() {
            let cur_sub = cur_line[i - 1] - cur_line[i];

            if cur_sub < 0 {
                only_decreasing = false;
            }
            if cur_sub > 0 {
                only_increasing = false;
            }

            if !(cur_sub.abs() >= 1 && cur_sub.abs() <= 3) {
                ok = false;
                break;
            }
        }
        if ok && (only_increasing || only_decreasing) {
            result += 1;
        }
    }

    println!("{result:?}");
}

fn is_safe_report(report: &[i64]) -> bool {
    let mut only_increasing = true;
    let mut only_decreasing = true;

    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];

        if !(1 <= diff.abs() && diff.abs() <= 3) {
            return false;
        }

        if diff < 0 {
            only_increasing = false;
        }
        if diff > 0 {
            only_decreasing = false;
        }
    }

    only_increasing || only_decreasing
}

fn part2(input: String) {
    let mut result = 0;

    for line in input.lines() {
        let cur_line: Vec<i64> = line
            .split_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect();

        if is_safe_report(&cur_line) {
            result += 1;
            continue;
        }

        let mut found_safe = false;
        for i in 0..cur_line.len() {
            let mut modified_report = cur_line.clone();
            modified_report.remove(i);

            if is_safe_report(&modified_report) {
                found_safe = true;
                break;
            }
        }

        if found_safe {
            result += 1;
        }
    }

    println!("{result}");
}

fn main() {
    let file = fs::read_to_string("./src/input.txt").unwrap();

    part1(file.clone());
    part2(file);
}
