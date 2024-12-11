use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file_path = "./input/8_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut lines = 0;
    let mut columns = 0;
    for (row, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        lines += 1;
        columns = 0;
        for (col, char) in line.chars().enumerate() {
            columns += 1;
            if char != '.' {
                map.entry(char).or_insert(vec![]).push((row, col));
            }
        }
    }

    let mut nodes_part1: HashSet<(i32, i32)> = HashSet::new();
    let mut nodes_part2: HashSet<(i32, i32)> = HashSet::new();
    for (_, positions) in map {
        let len = positions.len();
        for i in 0..len {
            let antenna_1 = (positions[i].0 as i32, positions[i].1 as i32);
            for j in 0..len {
                if i == j {
                    continue;
                }
                let antenna_2 = (positions[j].0 as i32, positions[j].1 as i32);
                let diff = (antenna_1.0 - antenna_2.0, antenna_1.1 - antenna_2.1);
                let mut new_node = (antenna_1.0 + diff.0, antenna_1.1 + diff.1);
                if (new_node.0 >= 0 && new_node.0 < lines)
                    && (new_node.1 >= 0 && new_node.1 < columns)
                {
                    nodes_part1.insert(new_node);
                    nodes_part2.insert(new_node);
                }
                
                let mut out_of_bounds = false;
                while !out_of_bounds {
                    new_node = (new_node.0 - diff.0, new_node.1 - diff.1);
                    if (new_node.0 >= 0 && new_node.0 < lines)
                        && (new_node.1 >= 0 && new_node.1 < columns)
                    {
                        nodes_part2.insert(new_node);
                    } else {
                        out_of_bounds = true;
                    }
                }
            }
        }
    }
    let part1_ans = nodes_part1.len();
    println!("Part 1: {part1_ans}");
    let part2_ans = nodes_part2.len();
    println!("Part 2: {part2_ans}");
}
