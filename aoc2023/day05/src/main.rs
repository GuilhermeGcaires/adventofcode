use std::{fs, ops::Range};

fn main() {
    let input = fs::read_to_string("./input/input.txt").expect("Could not read file");

    let (mappings, seeds) = parse(&input);

    part_1(&mappings, &seeds);
    part_2(&mappings, &seeds);
}

fn parse(input: &str) -> (Vec<Mapping>, Vec<i64>) {
    let mut mapping = Vec::new();
    let seeds: Vec<i64> = input
        .lines()
        .nth(0)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|d| d.parse::<i64>().expect("Could not parse digit"))
        .collect();

    let mut curmap = Mapping::default();
    for line in input.lines() {
        if line == "" {
            continue;
        }
        if line.contains(':') {
            mapping.push(curmap);
            curmap = Mapping::default();
            continue;
        }
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        curmap.add_mapping(nums[0], nums[1], nums[2]);
    }
    if !curmap.map.is_empty() {
        mapping.push(curmap);
    }
    (mapping, seeds)
}

fn part_1(mappings: &Vec<Mapping>, seeds: &Vec<i64>) {
    let mut min = i64::MAX;

    for seed in seeds {
        let mut cur = *seed;
        for map in mappings {
            cur = map.apply_map(cur);
        }
        min = min.min(cur);
    }
    println!("{:?}", min);
}

fn part_2(mappings: &Vec<Mapping>, seeds: &Vec<i64>) {
    let mut min = i64::MAX;

    for seed_range in seeds.chunks(2) {
        for seed in seed_range[0]..seed_range[0] + seed_range[1] {
            let mut cur = seed;
            for map in mappings {
                cur = map.apply_map(cur);
            }
            min = min.min(cur);
        }
    }
    println!("{:?}", min);
}

#[derive(Debug, Default, Clone)]
struct SingleMap {
    range: Range<i64>,
    delta: i64,
}

#[derive(Debug, Default)]
struct Mapping {
    map: Vec<SingleMap>,
}

impl Mapping {
    fn add_mapping(&mut self, dest: i64, src: i64, len: i64) {
        self.map.push(SingleMap {
            range: Range {
                start: src,
                end: src + len,
            },
            delta: dest - src,
        });
        self.map.sort_by_key(|r| r.range.start);
    }

    fn apply_map(&self, val: i64) -> i64 {
        for map in &self.map {
            if map.range.contains(&val) {
                return val + map.delta;
            }
        }
        val
    }
}
