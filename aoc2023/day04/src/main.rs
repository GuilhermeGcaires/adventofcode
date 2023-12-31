use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Card {
    id: u32,
    score: u32,
    numbers: Vec<u32>,
    drawn_numbers: Vec<u32>,
}

fn main() {
    let input = fs::read_to_string("./input/input.txt").expect("Unable to read file");

    let all_cards = parse(&input);
    process_part1(&all_cards);
    process_part2(&all_cards);
}

fn parse(input: &str) -> Vec<Card> {
    let mut all_cards: Vec<Card> = Vec::new();
    for line in input.lines() {
        let (card, numbers) = line.split_once(": ").unwrap();

        let card_num: u32 = card
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .expect("couldnt parse card_num");

        let (elf_nums, drawn_nums) = numbers.split_once(" | ").unwrap();
        println!("{:?}", elf_nums);
        let elf_nums: Vec<_> = elf_nums
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        let drawn_nums: Vec<_> = drawn_nums
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();

        let cur_card = Card {
            id: card_num,
            score: 0,
            numbers: elf_nums,
            drawn_numbers: drawn_nums,
        };
        all_cards.push(cur_card);
    }

    all_cards
}

fn process_part1(all_cards: &Vec<Card>) {
    let mut total_sum = 0;
    for card in all_cards {
        let mut cur_count = 0;
        for el in &card.numbers {
            if card.drawn_numbers.contains(&el) {
                cur_count += 1;
            }
        }
        println!("card id: {:?} count: {:?}", card.id, cur_count);
        if cur_count == 0 {
            continue;
        } else {
            total_sum += 2_u32.pow(cur_count - 1);
        }
    }
    println!("{:?}", total_sum);
}

fn process_part2(all_cards: &Vec<Card>) {
    let mut multiplier = vec![1usize; all_cards.len()];

    for (index, card) in all_cards.iter().enumerate() {
        let mut cur_count = 0;
        for el in &card.numbers {
            if card.drawn_numbers.contains(&el) {
                cur_count += 1;
            }
        }
        for i in index + 1..index + 1 + cur_count {
            multiplier[i] += multiplier[index];
        }
    }
    println!("{:?}", multiplier.iter().sum::<usize>());
}
