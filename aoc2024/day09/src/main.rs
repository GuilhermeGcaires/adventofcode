use std::{fs, result};

#[derive(Copy, Clone, Debug)]
struct Block {
    index: usize,
    size: usize,
    position: usize,
}

fn parse(input: &String) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut pos = 0;
    for (idx, ch) in input.trim().chars().enumerate() {
        let digit = ch.to_digit(10).unwrap() as usize;

        if idx % 2 == 0 {
            for i in 0..digit {
                blocks.push(Block {
                    index: idx / 2,
                    size: 1,
                    position: pos + i,
                });
            }
        }
        pos += digit;
    }

    blocks
}

fn part1(mut blocks: Vec<Block>) -> usize {
    loop {
        let mut last_block = blocks.pop().unwrap();

        let mut inserted = false;

        for i in 0..blocks.len() - 1 {
            let left_block = blocks[i].position + blocks[i].size;
            let right_block = blocks[i + 1].position;

            if right_block - left_block >= last_block.size {
                inserted = true;
                last_block.position = left_block;
                blocks.insert(i + 1, last_block);
                break;
            }
        }

        if !inserted {
            last_block.position = blocks.last().unwrap().position + blocks.last().unwrap().size;
            blocks.push(last_block);
            break;
        }
    }

    let mut result = 0;
    for block in &blocks {
        result += block.position * block.index;
    }

    result
}

fn parse2(input: &str) -> (Vec<Block>, Vec<Block>) {
    let mut files = Vec::new();
    let mut blanks = Vec::new();

    let mut index = 0;
    let mut pos = 0;

    for (i, ch) in input.trim().chars().enumerate() {
        let size = ch.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            if size == 0 {
                panic!("Unexpected size=0 for file");
            }
            files.push(Block {
                index,
                size,
                position: pos,
            });
            index += 1;
        } else {
            if size != 0 {
                blanks.push(Block {
                    index: usize::MAX, // Blanks have no file index
                    size,
                    position: pos,
                });
            }
        }
        pos += size;
    }

    (files, blanks)
}

fn part2(mut files: Vec<Block>, mut blanks: Vec<Block>) -> usize {
    for i in (0..files.len()).rev() {
        let file = files[i];
        let mut inserted = false;

        for j in 0..blanks.len() {
            let blank = blanks[j];

            if blank.position >= file.position {
                blanks.truncate(j);
                break;
            }

            if file.size <= blank.size {
                files[i].position = blank.position;

                if file.size == blank.size {
                    blanks.remove(j);
                } else {
                    blanks[j].position += file.size;
                    blanks[j].size -= file.size;
                }

                inserted = true;
                break;
            }
        }

        if !inserted {
            continue;
        }
    }

    files
        .iter()
        .map(|file| {
            (file.position..file.position + file.size)
                .map(|pos| file.index * pos)
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();

    let parsed_input = parse(&input);
    let (files, blanks) = parse2(&input);

    let result1 = part1(parsed_input);
    let result2 = part2(files, blanks);
    println!("{}", result1);
    println!("{}", result2);
}
