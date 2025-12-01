use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file_path = "./input/11_input.txt";
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let stones_vec: Vec<usize> = buffer
        .split(' ')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect();

    let mut stones: HashMap<usize, u128> = HashMap::new();
    for stone in stones_vec.iter() {
        stones.insert(*stone, 1);
    }

    let total_blinks = 75;
    let mut part_1_stones = 0;
    let mut part_2_stones = 0;
    for blink in 1..=total_blinks {
        let mut new_stones: HashMap<usize, u128> = HashMap::new();

        for (&stone, &count) in stones.iter() {
            if stone == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else if stone.to_string().len() % 2 == 0 {
                let digits = stone.to_string();
                let mid = digits.len() / 2;
                let first_half: usize = digits[0..mid].parse().unwrap();
                let second_half: usize = digits[mid..].parse().unwrap_or(0);
                *new_stones.entry(first_half).or_insert(0) += count;
                *new_stones.entry(second_half).or_insert(0) += count;
            } else {
                *new_stones.entry(stone * 2024).or_insert(0) += count;
            }
        }

        stones = new_stones;

        if blink == 25 {
            part_1_stones = stones.values().sum();
        }
        if blink == 75 {
            part_2_stones = stones.values().sum();
        }

        let current_count: u128 = stones.values().sum();
        println!("Blink #: {}, stones: {}", blink, current_count);
    }

    println!("Part 1: {part_1_stones}");
    println!("Part 2: {part_2_stones}");
}
