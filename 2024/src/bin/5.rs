use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file_path = "./input/5_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut done_parsing_rules = false;
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            done_parsing_rules = true;
            continue;
        }
        if !done_parsing_rules {
            if let Some((r1, r2)) = line.split_once('|') {
                let r1 = r1.trim().parse::<i32>().unwrap();
                let r2 = r2.trim().parse::<i32>().unwrap();
                rules
                    .entry(r1)
                    .and_modify(|value| value.push(r2))
                    .or_insert(vec![r2]);
            }
        } else {
            updates.push(
                line.split(',')
                    .filter_map(|item| item.trim().parse::<i32>().ok())
                    .collect(),
            );
        }
    }

    part_1(&updates, &rules);
    part_2(&mut updates, &rules);
}

fn part_1(updates: &Vec<Vec<i32>>, rules: &HashMap<i32, Vec<i32>>) {
    let mut total = 0;
    for update in updates.iter() {
        let mut is_update_correct = true;
        for (i, _) in update.iter().enumerate() {
            if !is_valid_page(i, &update, &rules).0 {
                is_update_correct = false;
            } else {
            }
        }
        if is_update_correct {
            total += update[update.len() / 2];
        }
    }
    println!("{total}");
}

fn part_2(updates: &mut Vec<Vec<i32>>, rules: &HashMap<i32, Vec<i32>>) {
    let mut invalid_updates = vec![];
    for (u, update) in updates.iter_mut().enumerate() {
        let mut i = 0;
        while i < update.len() {
            let is_valid = is_valid_page(i, &update, &rules);
            if !is_valid.0 {
                invalid_updates.push(u);
                update.swap(i, is_valid.1);
                i = 0;
            }
            i += 1;
        }
    }

    let mut total = 0;
    for (u, update) in updates.iter().enumerate() {
        if !invalid_updates.contains(&u) {
            continue;
        }
        total += update[update.len() / 2];
    }
    println!("{total}");
}

fn is_valid_page(i: usize, update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> (bool, usize) {
    let num = update[i];
    let rules_for_num = rules.get(&num).unwrap();
    for j in 0..i {
        let t = &update[j];
        if rules_for_num.contains(t) {
            return (false, j);
        }
    }
    (true, 0)
}
