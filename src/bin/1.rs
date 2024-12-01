use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file_path = "1_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);
    let mut row1 = vec![];
    let mut row2 = vec![];
    let mut map: HashMap<i32, i32> = HashMap::new();

    for line in reader.lines() {
        if let Some((num1, num2)) = line.unwrap().split_once("   ") {
            let num1 = num1.trim().parse::<i32>().unwrap();
            let num2 = num2.trim().parse::<i32>().unwrap();
            if map.contains_key(&num2) {
                let count = map.get(&num2).unwrap() + 1;
                map.insert(num2, count);
            } else {
                map.insert(num2, 1);
            }
            if !map.contains_key(&num1) {
                map.insert(num1, 0);
            }
            row1.push(num1);
            row2.push(num2);
        }
    }

    row1.sort();
    row2.sort();

    let mut total_diff = 0;
    let mut similarity_score = 0;
    for (i, n) in row1.iter().enumerate() {
        let diff = n.abs_diff(row2[i]);
        total_diff += diff;
        let n_similarity = n * map.get(n).unwrap();
        similarity_score += n_similarity;
    }

    println!("{}", total_diff);
    println!("{}", similarity_score);
}
