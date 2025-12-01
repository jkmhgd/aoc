use std::{
    fs::File,
    io::{self, BufRead},
    u64,
};

fn main() {
    let file_path = "./input/7_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut valid_tests_part1: Vec<u64> = vec![];
    let mut valid_tests_part2: Vec<u64> = vec![];
    for line in reader.lines() {
        if let Some((value, nums)) = line.unwrap().split_once(": ") {
            let value = value.trim().parse::<u64>().unwrap();
            let nums: Vec<u64> = nums
                .split(' ')
                .filter_map(|s| s.trim().parse::<u64>().ok())
                .collect();
            if dfs_part1(value, 0, 0, &nums) {
                valid_tests_part1.push(value);
            }
            if dfs_part2(value, 0, 0, &nums) {
                valid_tests_part2.push(value);
            }
        }
    }
    let sum_part1: u64 = valid_tests_part1.iter().sum();
    let sum_part2: u64 = valid_tests_part2.iter().sum();
    println!("{sum_part1}");
    println!("{sum_part2}");
}

fn dfs_part1(value: u64, current: u64, i: usize, nums: &Vec<u64>) -> bool {
    if i == nums.len() && value == current {
        return true;
    } else if i == nums.len() {
        return false;
    }
    let val_i = nums[i];
    let current_add = current + val_i;
    let current_mul = current * val_i;
    let i = i + 1;
    dfs_part1(value, current_add, i, nums) || dfs_part1(value, current_mul, i, nums)
}

fn dfs_part2(value: u64, current: u64, i: usize, nums: &Vec<u64>) -> bool {
    if i == nums.len() && value == current {
        return true;
    } else if i == nums.len() {
        return false;
    }
    let val_i = nums[i];
    let current_add = current + val_i;
    let current_mul = current * val_i;
    let current_cat = format!("{}{}", current, val_i).parse::<u64>().unwrap();
    let i = i + 1;
    dfs_part2(value, current_add, i, nums) || dfs_part2(value, current_mul, i, nums) || dfs_part2(value, current_cat, i, nums)
}
